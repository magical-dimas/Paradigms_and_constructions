use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn log_duration(args: TokenStream, item: TokenStream) -> TokenStream {
    log_duration_impl(args, item)
}

fn log_duration_impl(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let ItemFn {
        sig,
        vis,
        block,
        attrs,
    } = input;

    let statements = block.stmts;
    let function_identifier = sig.ident.clone();

    quote!(
        #(#attrs)*
        #vis #sig {
            let __start = std::time::Instant::now();
            let __result = {
                #(#statements)*
            };

            println!("{} - {}, dump:", stringify!(#function_identifier), __start.elapsed().as_micros());
            for s in &__result{
                println!("{} ", s);
            }
            println!("");
            return __result;
        }
    )
    .into()
}