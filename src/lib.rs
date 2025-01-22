/// Implementation of Python Functools Partial function.
/// Takes a function and arguments returning the function.
use quote::quote;
use syn::{parse::Parse, parse_macro_input, punctuated::Punctuated, Expr, Ident, Token};

struct Partial {
    // partial!(function, arg1, arg2, arg3...)
    function: Ident,
    args: Punctuated<Expr, Token![,]>,
}

impl Parse for Partial {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let function = input.parse()?;
        // Not sure why you would call partial with a no-args function but
        // we'll allow it.
        if input.is_empty() {
            return Ok(Self {
                function,
                args: Punctuated::new(),
            });
        }
        let _ = input.parse::<Token![,]>()?;
        let args = Punctuated::parse_terminated(input)?;
        Ok(Self { function, args })
    }
}

#[proc_macro]
pub fn partial(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let partial = parse_macro_input!(input as Partial);
    let func = partial.function;
    let args = partial.args.into_iter();
    quote! {#func(#(#args),*)}.into()
}
