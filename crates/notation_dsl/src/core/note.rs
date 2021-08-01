use fehler::{throws, throw};
use notation_proto::prelude::{Pitch, Semitones, Syllable};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, Parse, ParseStream};

use crate::context::Context;

use super::octave::OctaveTweakDsl;
use super::pitch_name::PitchNameDsl;
use super::pitch_sign::PitchSignDsl;

pub struct NoteDsl {
    pub octave_tweak: Option<OctaveTweakDsl>,
    pub pitch_sign: PitchSignDsl,
    pub pitch_name: PitchNameDsl,
}

impl Parse for NoteDsl {
    #[throws(Error)]
    fn parse(input: ParseStream) -> Self {
        let octave_tweak = OctaveTweakDsl::try_parse(input);
        let pitch_sign = input.parse::<PitchSignDsl>()?;
        let pitch_name = input.parse::<PitchNameDsl>()?;
        NoteDsl { octave_tweak, pitch_sign, pitch_name, }
    }
}

impl NoteDsl {
    pub fn peek(input: ParseStream) -> bool {
        OctaveTweakDsl::peek(input)
            || PitchSignDsl::peek(input)
            || PitchNameDsl::peek(input)
    }
}

impl ToTokens for NoteDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let NoteDsl { octave_tweak, pitch_sign, pitch_name, } = self;
        let octave_quote = Context::octave_quote(octave_tweak);
        if pitch_name.from_syllable {
            let syllable = Syllable::from(Semitones::from(pitch_sign.sign) + Semitones::from(pitch_name.name));
            let pitch_quote = Context::calc_pitch_quote(&syllable);
            let syllable_ident = syllable.to_ident();
            tokens.extend(quote! {
                Note::new(#octave_quote, #pitch_quote, Some(Syllable::from_ident(#syllable_ident)))
            });
        } else {
            let pitch = Pitch::new(pitch_name.name, pitch_sign.sign);
            let pitch_text = pitch.to_text();
            let syllable_quote = Context::calc_syllable_quote(&pitch);
            tokens.extend(quote! {
                Note::new(#octave_quote, Pitch::from_text(#pitch_text), Some(#syllable_quote))
            });
        }
    }
}
