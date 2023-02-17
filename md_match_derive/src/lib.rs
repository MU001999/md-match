//! # MdMatch
//! This crate provides the derive macro for md-match.
//! Please refer to https://github.com/Mu001999/md-match for how to set this up.

use proc_macro::TokenStream;
use proc_macro2::Span;

/// Derive MdMatch for Enum
#[proc_macro_derive(MdMatch)]
pub fn derive_md_match(item: TokenStream) -> TokenStream {
    let r#enum = syn::parse_macro_input!(item as syn::ItemEnum);

    let name = r#enum.ident;
    let generics = r#enum.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // clone generics as ref_generics and add lifetime 'mdmatch
    let mut ref_generics = generics.clone();
    ref_generics.params.push(syn::parse_quote!('mdmatch));
    // get the impl_generics of ref_generics
    let (ref_impl_generics, _, _) = ref_generics.split_for_impl();

    // get the element type, if no element, return token stream with error
    let elem_type = match r#enum
        .variants
        .iter()
        .map(|variant| {
            let fields = &variant.fields;
            if fields.len() == 1 {
                let field = fields.iter().next().unwrap();
                let ty = &field.ty;
                Some(ty)
            } else {
                None
            }
        })
        .next()
        .flatten()
    {
        Some(ty) => ty,
        None => {
            return syn::Error::new(Span::call_site(), "requires one element")
                .to_compile_error()
                .into()
        }
    };

    let variant = r#enum.variants.iter().map(|variant| &variant.ident);
    let variant_ref = variant.clone();
    let variant_mut_ref = variant.clone();

    // impl MdMatch for Enum， &Enum and &mut Enum
    let impl_md_match = quote::quote! {
        impl #impl_generics MdMatch for #name #ty_generics #where_clause {
            type Elem = #elem_type;
            fn md_match<R>(self, f: impl FnOnce(Self::Elem) -> R) -> R {
                match self {
                    #(#name::#variant(x) => f(x)),*
                }
            }
        }

        impl #ref_impl_generics MdMatch for &'mdmatch #name #ty_generics #where_clause {
            type Elem = &'mdmatch #elem_type;
            fn md_match<R>(self, f: impl FnOnce(Self::Elem) -> R) -> R {
                match self {
                    #(#name::#variant_ref(x) => f(x)),*
                }
            }
        }

        impl #ref_impl_generics MdMatch for &'mdmatch mut #name #ty_generics #where_clause {
            type Elem = &'mdmatch mut #elem_type;
            fn md_match<R>(self, f: impl FnOnce(Self::Elem) -> R) -> R {
                match self {
                    #(#name::#variant_mut_ref(x) => f(x)),*
                }
            }
        }
    };

    TokenStream::from(impl_md_match)
}
