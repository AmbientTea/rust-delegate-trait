use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, PatType, TypePath};
use syn::{
    FnArg, GenericParam, ItemTrait, Signature, TraitItemConst, TraitItemFn, TraitItemType, Type,
};

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
    let generics = &trait_item.generics;
    let params = &generics.params;
    let where_clause =
        (generics.where_clause.clone()).map_or_else(|| Punctuated::new(), |wc| wc.predicates);

    let delegated_trait_ident = Ident::new(
        &format!("Delegated{}", trait_ident.to_string()),
        Span::call_site(),
    );

    let item_impls = (trait_item.items.iter().cloned())
        .map(|item| implement_item(item, &trait_ident, &delegated_trait_ident, &params));

    quote! {
        #trait_item

        pub trait #delegated_trait_ident #generics {
            type DelegateType;

            fn delegate(self) -> Self::DelegateType;
            fn delegate_ref(&self) -> &Self::DelegateType;
            fn delegate_ref_mut(&mut self) -> &mut Self::DelegateType;
        }

        impl<
            Delegated_T: #delegated_trait_ident<#params>,
            #params
        >
            #trait_ident <#params> for Delegated_T
        where
            <Delegated_T as #delegated_trait_ident<#params>>::DelegateType: #trait_ident<#params>,
            #where_clause
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
    params: &Punctuated<GenericParam, Comma>,
) -> TokenStream {
    match item {
        syn::TraitItem::Fn(TraitItemFn { sig, .. }) => {
            implement_fn(sig, delegated_trait_ident, params)
        }

        syn::TraitItem::Const(TraitItemConst { ident, ty, .. }) => quote! {
            const #ident: #ty =
                <<Delegated_T as #delegated_trait_ident<#params>>::DelegateType as #trait_ident<#params>>::#ident;
        },

        syn::TraitItem::Type(TraitItemType { ident, .. }) => quote! {
            type #ident =
                <<Delegated_T as #delegated_trait_ident<#params>>::DelegateType as #trait_ident<#params>>::#ident;
        },

        syn::TraitItem::Macro(_) => quote!(compile_error!(
            "delegate-trait doesn't support macros as trait items"
        )),

        syn::TraitItem::Verbatim(verbatim) => verbatim,

        other => quote!(#other),
    }
}

fn implement_fn(
    sig: Signature,
    delegated_trait_ident: &Ident,
    params: &Punctuated<GenericParam, Comma>,
) -> TokenStream {
    let fn_ident = &sig.ident;
    let mut inputs: Vec<_> = sig.inputs.iter().collect();

    let delegated_fn = match inputs.first().and_then(|inp| SelfArg::try_from_arg(inp)) {
        None => quote!(<Delegated_T as #delegated_trait_ident<#params>>::DelegateType::#fn_ident),
        Some(self_arg) => {
            inputs.remove(0);
            match self_arg {
                SelfArg::Value => quote!(self.delegate().#fn_ident),
                SelfArg::Ref => quote!(self.delegate_ref().#fn_ident),
                SelfArg::MutRef => quote!(self.delegate_ref_mut().#fn_ident),
            }
        }
    };

    let inputs = inputs.into_iter().map(|input| match input {
        FnArg::Typed(PatType { pat, ty, .. }) => match SelfArg::try_from_type(ty.as_ref()) {
            Some(SelfArg::Ref) => quote!(#pat.delegate_ref()),
            Some(SelfArg::MutRef) => quote!(#pat.delegate_ref_mut()),
            Some(SelfArg::Value) => quote!(#pat.delegate()),
            None => quote!(#pat),
        },
        FnArg::Receiver(_) => unreachable!(),
    });

    quote! {
        #sig {
            #delegated_fn(#(#inputs)*)
        }
    }
}

fn path_is_self(path: &TypePath) -> bool {
    (path.path.segments.first().iter()).all(|seg| seg.ident.to_string() == "Self")
}

enum SelfArg {
    Value,
    Ref,
    MutRef,
}

impl SelfArg {
    pub fn new(isref: bool, ismut: bool) -> Self {
        match (isref, ismut) {
            (true, true) => Self::MutRef,
            (true, false) => Self::Ref,
            _ => Self::Value,
        }
    }
    fn try_from_type(ty: &Type) -> Option<Self> {
        match ty {
            Type::Reference(reff) if Self::try_from_type(reff.elem.as_ref()).is_some() => {
                Some(SelfArg::new(true, reff.mutability.is_some()))
            }
            Type::Path(path) if path_is_self(&path) => Some(SelfArg::new(false, false)),
            _ => None,
        }
    }

    fn try_from_arg(arg: &FnArg) -> Option<Self> {
        match arg {
            FnArg::Receiver(recv) => Some(Self::new(
                recv.reference.is_some(),
                recv.mutability.is_some(),
            )),
            _ => None,
        }
    }
}
