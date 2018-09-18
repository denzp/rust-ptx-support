use proc_macro2::{Ident, Span, TokenStream};
use quote::*;

use syn::parse::{self, Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Expr, LitStr, Token};

use crate::error::PrintSyscallError;
use crate::parsers::*;

pub struct MacroInputs {
    pub format: String,
    pub format_span: Span,
    pub args: Vec<Expr>,
}

#[derive(Debug)]
pub struct WrappedArg {
    pub inner_name: Ident,
    pub inner_ty: TokenStream,
    pub inner_generic: Option<Ident>,

    pub ffi_ty: TokenStream,
    pub ffi_expr: TokenStream,
}

pub fn wrap_args(formatted: &[Arg], exprs: &[Expr]) -> Result<Vec<WrappedArg>, PrintSyscallError> {
    let exprs_iter = exprs.iter();
    let formatted_iter = formatted.iter().filter(|item| {
        if let Arg(_, _, _, _, ArgType::Literal) = item {
            false
        } else {
            true
        }
    });

    let exprs_count = exprs_iter.clone().count();
    let formatted_count = formatted_iter.clone().count();

    if exprs_count != formatted_count {
        return Err(PrintSyscallError::WrongArgumentsCount {
            expected: formatted_count,
            got: exprs_count,
        });
    }

    exprs_iter
        .zip(formatted_iter)
        .zip(0..exprs.len())
        .map(|((expr, arg), index)| {
            let inner_name = Ident::new(&format!("arg_{}", index), expr.span());
            let generic_arg_name = Ident::new(&format!("T{}", index), expr.span());

            match arg {
                Arg(_, _, _, None, ArgType::Signed) => Ok(WrappedArg {
                    ffi_expr: quote_spanned! { expr.span() => #inner_name },
                    ffi_ty: quote_spanned! { expr.span() => i32 },

                    inner_ty: quote_spanned! { expr.span() => i32 },
                    inner_generic: None,
                    inner_name,
                }),

                Arg(_, _, _, Some(ArgLength::I8), ArgType::Signed) => Ok(WrappedArg {
                    ffi_expr: quote_spanned! { expr.span() => #inner_name },
                    ffi_ty: quote_spanned! { expr.span() => i8 },

                    inner_ty: quote_spanned! { expr.span() => i8 },
                    inner_generic: None,
                    inner_name,
                }),

                Arg(_, _, _, Some(ArgLength::I16), ArgType::Signed) => Ok(WrappedArg {
                    ffi_expr: quote_spanned! { expr.span() => #inner_name },
                    ffi_ty: quote_spanned! { expr.span() => i16 },

                    inner_ty: quote_spanned! { expr.span() => i16 },
                    inner_generic: None,
                    inner_name,
                }),

                Arg(_, _, _, Some(ArgLength::I64), ArgType::Signed) => Ok(WrappedArg {
                    ffi_expr: quote_spanned! { expr.span() => #inner_name },
                    ffi_ty: quote_spanned! { expr.span() => i64 },

                    inner_ty: quote_spanned! { expr.span() => i64 },
                    inner_generic: None,
                    inner_name,
                }),

                Arg(_, _, _, None, ArgType::Unsigned) => Ok(WrappedArg {
                    ffi_expr: quote_spanned! { expr.span() => #inner_name },
                    ffi_ty: quote_spanned! { expr.span() => u32 },

                    inner_ty: quote_spanned! { expr.span() => u32 },
                    inner_generic: None,
                    inner_name,
                }),

                Arg(_, _, _, Some(ArgLength::I8), ArgType::Unsigned) => Ok(WrappedArg {
                    ffi_expr: quote_spanned! { expr.span() => #inner_name },
                    ffi_ty: quote_spanned! { expr.span() => u8 },

                    inner_ty: quote_spanned! { expr.span() => u8 },
                    inner_generic: None,
                    inner_name,
                }),

                Arg(_, _, _, Some(ArgLength::I16), ArgType::Unsigned) => Ok(WrappedArg {
                    ffi_expr: quote_spanned! { expr.span() => #inner_name },
                    ffi_ty: quote_spanned! { expr.span() => u16 },

                    inner_ty: quote_spanned! { expr.span() => u16 },
                    inner_generic: None,
                    inner_name,
                }),

                Arg(_, _, _, Some(ArgLength::I64), ArgType::Unsigned) => Ok(WrappedArg {
                    ffi_expr: quote_spanned! { expr.span() => #inner_name },
                    ffi_ty: quote_spanned! { expr.span() => u64 },

                    inner_ty: quote_spanned! { expr.span() => u64 },
                    inner_generic: None,
                    inner_name,
                }),

                Arg(_, _, _, None, ArgType::Char) => Ok(WrappedArg {
                    ffi_expr: quote_spanned! { expr.span() => #inner_name },
                    ffi_ty: quote_spanned! { expr.span() => i8 },

                    inner_ty: quote_spanned! { expr.span() => char },
                    inner_generic: None,
                    inner_name,
                }),

                Arg(_, _, _, None, ArgType::Double) => Ok(WrappedArg {
                    ffi_expr: quote_spanned! { expr.span() => #inner_name },
                    ffi_ty: quote_spanned! { expr.span() => f64 },

                    inner_ty: quote_spanned! { expr.span() => f64 },
                    inner_generic: None,
                    inner_name,
                }),

                Arg(_, _, _, None, ArgType::StringPointer) => Ok(WrappedArg {
                    ffi_expr: quote_spanned! { expr.span() => #inner_name.as_ref().as_ptr() },
                    ffi_ty: quote_spanned! { expr.span() => *const u8 },

                    inner_ty: quote_spanned! { expr.span() => impl AsRef<str> },
                    inner_generic: None,
                    inner_name,
                }),

                Arg(_, _, _, None, ArgType::Pointer) => Ok(WrappedArg {
                    ffi_expr: quote_spanned! { expr.span() => #inner_name as *const _ },
                    ffi_ty: quote_spanned! { expr.span() => *const u8 },

                    inner_ty: quote_spanned! { expr.span() => *const #generic_arg_name },
                    inner_generic: Some(generic_arg_name),
                    inner_name,
                }),

                Arg(_, _, _, _, ArgType::Literal) => {
                    unreachable!("literals should be handled independently");
                }

                Arg(_, _, _, Some(_), ty) => {
                    Err(PrintSyscallError::UnacceptableLength { for_type: *ty })
                }
            }
        }).try_fold(Vec::with_capacity(exprs.len()), |mut result, item| {
            result.push(item?);

            Ok(result)
        })
}

impl Parse for MacroInputs {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let format = input.parse::<LitStr>()?;

        let args = if !input.is_empty() {
            input.parse::<Token![,]>()?;

            Punctuated::<Expr, Token![,]>::parse_terminated(input)?
                .iter()
                .cloned()
                .collect()
        } else {
            vec![]
        };

        Ok(MacroInputs {
            format: format.value(),
            format_span: format.span(),
            args,
        })
    }
}

#[cfg(test)]
mod tests {
    use proc_macro2::{Ident, TokenStream};
    use quote::*;
    use syn::parse_quote;

    use super::wrap_args;
    use crate::error::PrintSyscallError;
    use crate::parsers::*;

    #[test]
    fn test_default_length() {
        let wrapped = wrap_args(
            &[
                Arg(None, None, None, None, ArgType::Literal),
                Arg(None, None, None, None, ArgType::Signed),
                Arg(None, None, None, None, ArgType::Unsigned),
                Arg(None, None, None, None, ArgType::Char),
                Arg(None, None, None, None, ArgType::Double),
                Arg(None, None, None, None, ArgType::StringPointer),
                Arg(None, None, None, None, ArgType::Pointer),
            ],
            &[
                parse_quote!(a),
                parse_quote!(a),
                parse_quote!(a),
                parse_quote!(a),
                parse_quote!(a),
                parse_quote!(a),
            ],
        ).unwrap();

        assert_eq!(
            stringify_idents(wrapped.iter().map(|item| item.inner_name.clone())),
            vec![
                "arg_0".to_owned(),
                "arg_1".to_owned(),
                "arg_2".to_owned(),
                "arg_3".to_owned(),
                "arg_4".to_owned(),
                "arg_5".to_owned(),
            ],
        );

        assert_eq!(
            stringify_token_streams(wrapped.iter().map(|item| item.inner_ty.clone())),
            vec![
                "i32".to_owned(),
                "u32".to_owned(),
                "char".to_owned(),
                "f64".to_owned(),
                "impl AsRef < str >".to_owned(),
                "* const T5".to_owned(),
            ],
        );

        assert_eq!(
            stringify_token_streams(wrapped.iter().map(|item| item.ffi_ty.clone())),
            vec![
                "i32".to_owned(),
                "u32".to_owned(),
                "i8".to_owned(),
                "f64".to_owned(),
                "* const u8".to_owned(),
                "* const u8".to_owned(),
            ],
        );

        assert_eq!(
            stringify_token_streams(wrapped.iter().map(|item| item.ffi_expr.clone())),
            vec![
                "arg_0".to_owned(),
                "arg_1".to_owned(),
                "arg_2".to_owned(),
                "arg_3".to_owned(),
                "arg_4 . as_ref ( ) . as_ptr ( )".to_owned(),
                "arg_5 as * const _".to_owned(),
            ],
        );
    }

    #[test]
    fn test_different_length() {
        let wrapped = wrap_args(
            &[
                Arg(None, None, None, Some(ArgLength::I8), ArgType::Signed),
                Arg(None, None, None, Some(ArgLength::I8), ArgType::Unsigned),
                Arg(None, None, None, Some(ArgLength::I16), ArgType::Signed),
                Arg(None, None, None, Some(ArgLength::I16), ArgType::Unsigned),
                Arg(None, None, None, Some(ArgLength::I64), ArgType::Signed),
                Arg(None, None, None, Some(ArgLength::I64), ArgType::Unsigned),
            ],
            &[
                parse_quote!(a),
                parse_quote!(a),
                parse_quote!(a),
                parse_quote!(a),
                parse_quote!(a),
                parse_quote!(a),
            ],
        ).unwrap();

        assert_eq!(
            stringify_token_streams(wrapped.iter().map(|item| item.inner_ty.clone())),
            vec![
                "i8".to_owned(),
                "u8".to_owned(),
                "i16".to_owned(),
                "u16".to_owned(),
                "i64".to_owned(),
                "u64".to_owned(),
            ],
        );
    }

    #[test]
    fn test_unacceptable_length() {
        assert_eq!(
            wrap_args(
                &[Arg(None, None, None, Some(ArgLength::I8), ArgType::Char)],
                &[parse_quote!(a)],
            ).unwrap_err(),
            PrintSyscallError::UnacceptableLength {
                for_type: ArgType::Char
            }
        );

        assert_eq!(
            wrap_args(
                &[Arg(None, None, None, Some(ArgLength::I8), ArgType::Double)],
                &[parse_quote!(a)],
            ).unwrap_err(),
            PrintSyscallError::UnacceptableLength {
                for_type: ArgType::Double
            }
        );

        assert_eq!(
            wrap_args(
                &[Arg(
                    None,
                    None,
                    None,
                    Some(ArgLength::I8),
                    ArgType::StringPointer
                )],
                &[parse_quote!(a)],
            ).unwrap_err(),
            PrintSyscallError::UnacceptableLength {
                for_type: ArgType::StringPointer
            }
        );

        assert_eq!(
            wrap_args(
                &[Arg(None, None, None, Some(ArgLength::I8), ArgType::Pointer)],
                &[parse_quote!(a)],
            ).unwrap_err(),
            PrintSyscallError::UnacceptableLength {
                for_type: ArgType::Pointer
            }
        );
    }

    #[test]
    fn test_extra_or_missing_args() {
        assert_eq!(
            {
                wrap_args(
                    &[
                        Arg(None, None, None, None, ArgType::Literal),
                        Arg(None, None, None, None, ArgType::Signed),
                        Arg(None, None, None, None, ArgType::Unsigned),
                    ],
                    &[parse_quote!(a), parse_quote!(a), parse_quote!(a)],
                ).unwrap_err()
            },
            PrintSyscallError::WrongArgumentsCount {
                expected: 2,
                got: 3,
            },
        );

        assert_eq!(
            {
                wrap_args(
                    &[
                        Arg(None, None, None, None, ArgType::Literal),
                        Arg(None, None, None, None, ArgType::Signed),
                        Arg(None, None, None, None, ArgType::Unsigned),
                    ],
                    &[parse_quote!(a)],
                ).unwrap_err()
            },
            PrintSyscallError::WrongArgumentsCount {
                expected: 2,
                got: 1,
            },
        );
    }

    fn stringify_idents(iter: impl Iterator<Item = Ident>) -> Vec<String> {
        iter.map(|item| item.to_string()).collect()
    }

    fn stringify_token_streams(iter: impl Iterator<Item = TokenStream>) -> Vec<String> {
        iter.map(|item| item.to_string()).collect()
    }
}
