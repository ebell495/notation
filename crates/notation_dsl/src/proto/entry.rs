use fehler::{throw, throws};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, ParseStream};
use syn::{Ident, LitStr, Token};

use crate::context::ContextDsl;
use crate::core::tone::ToneDsl;
use crate::fretted::fretboard::FretboardDsl;
use crate::fretted::pick::PickDsl;
use crate::fretted::shape::ShapeDsl;
use crate::proto::mark::MarkDsl;

pub struct MultibleDsl<T> {
    pub items: Vec<T>,
}

pub enum EntryDsl {
    Context(ContextDsl),
    Mark(MarkDsl),
    Tone(MultibleDsl<ToneDsl>),
    Pick(MultibleDsl<PickDsl>),
    Shape(MultibleDsl<ShapeDsl>),
    Fretboard(FretboardDsl),
}

impl EntryDsl {
    #[throws(Error)]
    pub fn parse_without_brace(input: ParseStream) -> Self {
        if ContextDsl::peek(input) {
            Self::Context(input.parse()?)
        } else if MarkDsl::peek(input) {
            Self::Mark(input.parse()?)
        } else {
            match input.parse::<Ident>()?.to_string().as_str() {
                "Tone" => Self::Tone(input.parse()?),
                "Pick" => Self::Pick(input.parse()?),
                "Shape" => Self::Shape(input.parse()?),
                "Fretboard" => Self::Fretboard(input.parse()?),
                _ => throw!(Error::new(input.span(), "Invalid Entry")),
            }
        }
    }
}

impl ToTokens for EntryDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Self::Mark(x) => quote! { ProtoEntry::from(#x) },
            Self::Context(x) => quote! { #x },
            Self::Tone(x) => quote! { #x },
            Self::Pick(x) => quote! { #x },
            Self::Shape(x) => quote! { #x },
            Self::Fretboard(x) => quote! { #x },
        });
    }
}
