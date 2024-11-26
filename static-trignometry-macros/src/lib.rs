extern crate proc_macro;

use darling::FromDeriveInput;
use num_traits::{
    float::{Float, FloatConst},
    ToBytes,
};
use quote::{quote, ToTokens};
use syn::{parse::Parse, parse_macro_input, DeriveInput, GenericParam, LitInt};

struct Length(usize);

impl Parse for Length {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lit = input.parse::<LitInt>()?;
        let length: usize = lit.base10_parse()?;
        Ok(Length(length))
    }
}

fn sin_literals_in_be_bytes<T: Float + FloatConst + ToBytes>(
    samples: usize,
) -> impl Iterator<Item = proc_macro2::TokenStream> {
    let step = T::FRAC_PI_2() / T::from(samples).unwrap();
    (0..samples)
        .map(move |i| (step * T::from(i).unwrap()).sin())
        .map(|v| v.to_be_bytes())
        .map(|bytes| {
            let bytes = bytes.as_ref();
            quote!([#(#bytes),*])
        })
}

#[proc_macro]
pub fn f32_sine_values(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Length(length) = parse_macro_input!(tokens as Length);
    let literals = sin_literals_in_be_bytes::<f32>(length);
    quote! {
        [#(unsafe { core::mem::transmute::<i32, f32>(i32::from_be_bytes(#literals)) }),*]
    }
    .into()
}

#[proc_macro]
pub fn f64_sine_values(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Length(length) = parse_macro_input!(tokens as Length);
    let literals = sin_literals_in_be_bytes::<f64>(length);
    quote! {
        [#(unsafe { core::mem::transmute::<i64, f64>(i64::from_be_bytes(#literals)) }),*]
    }
    .into()
}

#[derive(FromDeriveInput)]
#[darling(attributes(trig))]
struct Trig {
    samples: usize,
}

#[proc_macro_derive(StaticTrigF32, attributes(trig))]
#[allow(non_snake_case)]
pub fn derive_StaticTrigF32(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let inputs = parse_macro_input!(tokens as DeriveInput);
    let Trig { samples } = Trig::from_derive_input(&inputs).expect("wrong options");
    let DeriveInput {
        ident, generics, ..
    } = inputs;
    let where_clause = generics.where_clause;
    let generic_with_bounds = generics.params.iter();
    let generic_names = generics.params.iter().map(|p| match p {
        GenericParam::Type(ty) => ty.ident.to_token_stream(),
        GenericParam::Lifetime(l) => l.lifetime.to_token_stream(),
        GenericParam::Const(c) => c.ident.to_token_stream(),
    });

    quote! {
        impl<#(#generic_with_bounds),*> StaticTrignometry<#samples> for #ident<#(#generic_names),*>
            #where_clause
        {
            type FloatType = f32;
            const QUARTER_SINE: [Self::FloatType; #samples] = f32_sine_values!(#samples);
        }
    }
    .into()
}

#[proc_macro_derive(StaticTrigF64, attributes(trig))]
#[allow(non_snake_case)]
pub fn derive_StaticTrigF64(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let inputs = parse_macro_input!(tokens as DeriveInput);
    let Trig { samples } = Trig::from_derive_input(&inputs).expect("wrong options");
    let DeriveInput {
        ident, generics, ..
    } = inputs;
    let where_clause = generics.where_clause;
    let generic_with_bounds = generics.params.iter();
    let generic_names = generics.params.iter().map(|p| match p {
        GenericParam::Type(ty) => ty.ident.to_token_stream(),
        GenericParam::Lifetime(l) => l.lifetime.to_token_stream(),
        GenericParam::Const(c) => c.ident.to_token_stream(),
    });

    quote! {
        impl<#(#generic_with_bounds),*> StaticTrignometry<#samples> for #ident<#(#generic_names),*>
            #where_clause
        {
            type FloatType = f64;
            const QUARTER_SINE: [Self::FloatType; #samples] = f64_sine_values!(#samples);
        }
    }
    .into()
}
