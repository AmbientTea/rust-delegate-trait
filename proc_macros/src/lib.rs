use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::FnArg;
use syn::ItemTrait;
use syn::Receiver;
use syn::Signature;
use syn::TraitItemConst;
use syn::TraitItemFn;
use syn::TraitItemType;

#[proc_macro_attribute]
pub fn delegated(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = parse_macro_input!(input as ItemTrait);

    delegated_impl(&item).into()
}

fn delegated_impl(trait_item: &syn::ItemTrait) -> TokenStream {
    let trait_ident = &trait_item.ident;

    let delegated_trait_ident = Ident::new(
        &format!("Delegated{}", trait_ident.to_string()),
        Span::call_site(),
    );

    let item_impls = (trait_item.items.iter().cloned())
        .map(|item| implement_item(item, &trait_ident, &delegated_trait_ident));

    quote! {
        #trait_item

        pub trait #delegated_trait_ident {
            type DelegateType;

            fn delegate(self) -> Self::DelegateType;
            fn delegate_ref(&self) -> &Self::DelegateType;
            fn delegate_ref_mut(&mut self) -> &mut Self::DelegateType;
        }

        impl<T: #delegated_trait_ident> #trait_ident for T
        where
        <T as #delegated_trait_ident>::DelegateType: #trait_ident
        {
            #(#item_impls)*
        }
    }
    .into()
}

fn implement_item(
    item: syn::TraitItem,
    trait_ident: &Ident,
    delegated_trait_ident: &Ident,
) -> TokenStream {
    match item {
        syn::TraitItem::Fn(TraitItemFn { sig, .. }) => implement_fn(sig, delegated_trait_ident),

        syn::TraitItem::Const(TraitItemConst { ident, ty, .. }) => quote! {
            const #ident: #ty =
            <<T as #delegated_trait_ident>::DelegateType as #trait_ident>::#ident;
        },

        syn::TraitItem::Type(TraitItemType { ident, .. }) => quote! {
            type #ident =
                <<T as #delegated_trait_ident>::DelegateType as #trait_ident>::#ident;
        },

        syn::TraitItem::Macro(_) => quote!(compile_error!(
            "delegate-trait doesn't support macros as trait items"
        )),
        syn::TraitItem::Verbatim(verbatim) => verbatim,
        other => quote!(#other),
    }
}

fn implement_fn(sig: Signature, delegated_trait_ident: &Ident) -> TokenStream {
    let fn_ident = &sig.ident;
    let mut inputs: Vec<_> = sig.inputs.iter().collect();

    let delegate = if let Some(FnArg::Receiver(Receiver {
        reference,
        mutability,
        ..
    })) = inputs.first()
    {
        inputs.remove(0);
        match (reference.is_some(), mutability.is_some()) {
            (true, true) => quote!(self.delegate_ref_mut().#fn_ident),
            (true, false) => quote!(self.delegate_ref().#fn_ident),
            _ => quote!(self.delegate().#fn_ident),
        }
    } else {
        quote!(<T as #delegated_trait_ident>::DelegateType::#fn_ident)
    };

    let inputs = inputs.into_iter().map(|input| match input {
        syn::FnArg::Typed(arg) => arg.pat.clone(),
        syn::FnArg::Receiver(_) => unreachable!(),
    });

    quote! {
        #sig {
            #delegate(#(#inputs)*)
        }
    }
}
