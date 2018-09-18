#![deny(warnings)]
#![feature(proc_macro_diagnostic)]

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::*;
use syn::parse_macro_input;

mod args;
mod error;
mod parsers;

use crate::args::{wrap_args, MacroInputs};
use crate::parsers::parse_format;

#[proc_macro]
pub fn cuda_printf(input: TokenStream) -> TokenStream {
    let MacroInputs {
        format,
        format_span,
        args,
    } = parse_macro_input!(input as MacroInputs);

    let parsed_format_args = match parse_format(&format) {
        Ok(args) => args,

        Err(error) => {
            format_span
                .unstable()
                .error("Unable to parse `printf` format")
                .help(error.to_string())
                .emit();

            return TokenStream::from(quote!{});
        }
    };

    let wrapped_args = match wrap_args(&parsed_format_args, &args) {
        Ok(args) => args,

        Err(error) => {
            Span::call_site().unstable().error(error.to_string()).emit();

            return TokenStream::from(quote!{});
        }
    };

    let arg_types = wrapped_args.iter().map(|item| item.inner_ty.clone());
    let arg_names = wrapped_args.iter().map(|item| item.inner_name.clone());
    let ffi_types = wrapped_args.iter().map(|item| item.ffi_ty.clone());
    let ffi_args = wrapped_args.iter().map(|item| item.ffi_expr.clone());

    let arg_generics = wrapped_args
        .iter()
        .map(|item| item.inner_generic.clone())
        .filter_map(|item| item);

    TokenStream::from(quote! {{
        extern "C" {
            pub fn vprintf(format: *const u8, valist: *const u8) -> i32;
        }

        #[repr(C)]
        struct LocalPrintfArgs(#(#ffi_types),*);

        fn local_typed_vprintf<#(#arg_generics),*>(format: *const u8, #(#arg_names: #arg_types),*) {
            let args = LocalPrintfArgs(#(#ffi_args),*);

            unsafe {
                vprintf(format, ::core::mem::transmute(&args));
            }
        }

        local_typed_vprintf(#format.as_ptr(), #(#args),*);
    }})
}
