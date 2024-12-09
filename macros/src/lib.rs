use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Expr, Token};

struct AocInput {
    year: Expr,
    day: Expr,
    part: Expr,
    input: Expr,
}

impl Parse for AocInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let year: Expr = input.parse()?;
        input.parse::<Token![,]>()?;
        let day: Expr = input.parse()?;
        input.parse::<Token![,]>()?;
        let part: Expr  = input.parse()?;
        input.parse::<Token![,]>()?;
        let input: Expr = input.parse()?;

        Ok(Self {
            year,
            day,
            part,
            input
        })
    }
}

#[proc_macro_attribute]
pub fn aoc_solver(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AocInput);
    let function  = syn::parse::<syn::ItemFn>(item).expect("Attribute only supports functions");
    let solver_ident = function.sig.ident.clone();
    let variable_ident = quote::format_ident!("{}_REGISTRATION", solver_ident.to_string().to_uppercase());

    let year = args.year;
    let day = args.day;
    let part = args.part;
    let input = args.input;

    let gen = quote! {
        #[linkme::distributed_slice(crate::SOLVERS)]
        static #variable_ident: crate::SolverMetadata<'static> = crate::SolverMetadata {
            year: #year,
            day: #day,
            part: #part,
            input: #input,
            func: #solver_ident,
        };
        #function
    };

    gen.into()
}