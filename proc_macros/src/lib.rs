use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::FnArg;
use syn::ItemTrait;

#[proc_macro_attribute]
pub fn delegated(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = parse_macro_input!(input as ItemTrait);

    delegated_impl(&item).into()
}

fn delegated_impl(def: &syn::ItemTrait) -> TokenStream {
    let trait_ident = &def.ident;

    let delegated_trait_ident = Ident::new(
        &format!("Delegated{}", trait_ident.to_string()),
        Span::call_site(),
    );

    let item_impls = (def.items.iter().cloned())
        .map(|item| implement_item(item, &def, delegated_trait_ident.clone()));

    quote! {
        #def

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
    def: &syn::ItemTrait,
    delegated_trait_ident: Ident,
) -> TokenStream {
    let implementation = match item {
        syn::TraitItem::Fn(trait_item_fn) => implement_fn(trait_item_fn),
        syn::TraitItem::Const(trait_item_const) => {
            implement_const(trait_item_const, def, delegated_trait_ident)
        }
        syn::TraitItem::Type(trait_item_type) => {
            implement_type(trait_item_type, def, delegated_trait_ident)
        }
        syn::TraitItem::Macro(_) => quote!(compile_error!(
            "delegate-trait doesn't support macros as trait items"
        )),
        syn::TraitItem::Verbatim(verbatim) => verbatim,
        other => quote!(#other),
    };

    implementation
}

fn implement_fn(item_fn: syn::TraitItemFn) -> TokenStream {
    let sig = item_fn.sig;
    let fn_name = sig.ident.clone();
    let inputs: Vec<_> = sig.inputs.iter().collect();
    let Some((FnArg::Receiver(receiver), inputs)) = inputs.split_first() else {
        let error_msg = format!("Trait function {fn_name} must have a `self` receiver.");
        return quote!(compile_error(#error_msg));
    };

    let delegate = match (&receiver.reference, &receiver.mutability) {
        (Some(_), Some(_)) => quote!(delegate_ref_mut()),
        (Some(_), None) => quote!(delegate_ref()),
        _ => quote!(delegate()),
    };

    let inputs = inputs.into_iter().map(|input| match input {
        syn::FnArg::Typed(arg) => arg.pat.clone(),
        syn::FnArg::Receiver(_) => unreachable!(),
    });

    quote! {
        #sig {
            self.#delegate.#fn_name(#(#inputs)*)
        }
    }
}

fn implement_const(
    item: syn::TraitItemConst,
    def: &syn::ItemTrait,
    delegated_trait_ident: Ident,
) -> TokenStream {
    let const_name = item.ident;
    let const_type = item.ty;
    let trait_name = &def.ident;
    quote! {
        const #const_name: #const_type =
        <<T as #delegated_trait_ident>::DelegateType as #trait_name>::#const_name;
    }
}

fn implement_type(
    item: syn::TraitItemType,
    def: &syn::ItemTrait,
    delegated_trait_ident: Ident,
) -> TokenStream {
    let type_name = item.ident;
    let trait_name = &def.ident;

    quote! {
        type #type_name =
            <<T as #delegated_trait_ident>::DelegateType as #trait_name>::#type_name;
    }
}
