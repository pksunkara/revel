use crate::router::ty::subty_if_name;
use syn::{
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    token::{At, Brace, Bracket, Colon, Div, Question, Star},
    Ident, LitStr, Type,
};

pub struct PathSegmentDynamic {
    pub ident: Ident,
    pub optional: bool,
    pub glob: bool,
    pub ty: Option<Type>,
    pub regex: Option<LitStr>,
}

impl PathSegmentDynamic {
    fn new(ident: Ident) -> Self {
        Self {
            ident,
            optional: false,
            glob: false,
            ty: None,
            regex: None,
        }
    }
}

pub enum PathSegment {
    Static(LitStr),
    Dynamic(PathSegmentDynamic),
}

impl Parse for PathSegment {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(LitStr) {
            let lit: LitStr = input.parse()?;
            // TODO:(router) only allow url encoded strings
            Ok(PathSegment::Static(lit))
        } else {
            let mut dynamic = PathSegmentDynamic::new(input.parse()?);

            if input.peek(Question) {
                input.parse::<Question>()?;
                dynamic.optional = true;
            } else if input.peek(Star) {
                input.parse::<Star>()?;
                dynamic.glob = true;

                if input.peek(Question) {
                    input.parse::<Question>()?;
                    dynamic.optional = true;
                }
            } else {
                if input.peek(Colon) {
                    input.parse::<Colon>()?;
                    dynamic.ty = Some(input.parse()?);
                }

                if input.peek(At) {
                    input.parse::<At>()?;
                    dynamic.regex = Some(input.parse()?);
                }

                if let Some(ty) = dynamic.ty.clone() {
                    if let Some(ty) = subty_if_name(ty.clone(), "Vec") {
                        dynamic.glob = true;
                        dynamic.ty = Some(ty);
                    } else if let Some(ty) = subty_if_name(ty.clone(), "Option") {
                        dynamic.optional = true;

                        if let Some(ty) = subty_if_name(ty.clone(), "Vec") {
                            dynamic.glob = true;
                            dynamic.ty = Some(ty);
                        } else {
                            dynamic.ty = Some(ty);
                        }
                    }
                }
            }

            Ok(PathSegment::Dynamic(dynamic))
        }
    }
}

pub struct Path {
    pub segments: Punctuated<PathSegment, Div>,
}

impl Parse for Path {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Path {
            segments: {
                if input.peek(Brace) || input.peek(Bracket) {
                    Punctuated::new()
                } else {
                    Punctuated::parse_separated_nonempty_with(input, |i| i.parse::<PathSegment>())?
                }
            },
        })
    }
}
