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
    let item_impls = def
        .items
        .iter()
        .cloned()
        .map(|item| implement_item(item, &def));

    quote! {
        #def

        impl<T: Delegated<dyn #trait_ident>> #trait_ident for T
        where
            <T as Delegated<dyn #trait_ident>>::DelegateType: #trait_ident
        {
            #(#item_impls)*
        }
    }
    .into()
}

fn implement_item(item: syn::TraitItem, _def: &syn::ItemTrait) -> TokenStream {
    let implementation = match item {
        syn::TraitItem::Fn(trait_item_fn) => implement_fn(trait_item_fn),
        syn::TraitItem::Const(_) => quote!(compile_error!(
            "delegate-trait doesn't support constants as trait items"
        )),
        syn::TraitItem::Type(_) => quote!(compile_error!(
            "delegate-trait doesn't support types as trait items"
        )),
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
