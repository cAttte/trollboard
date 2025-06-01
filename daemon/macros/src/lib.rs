use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, ItemFn, Lit, Meta, MetaNameValue, ExprLit};

#[proc_macro_attribute]
pub fn button(attr: TokenStream, item: TokenStream) -> TokenStream {
    let meta = parse_macro_input!(attr as Meta);
    let input_fn = parse_macro_input!(item as ItemFn).clone();

    let fn_name = input_fn.sig.ident.clone();
    let fn_name_str = fn_name.to_string();

    let description = match meta {
        Meta::NameValue(MetaNameValue { value: Expr::Lit(ExprLit { lit: Lit::Str(s), .. }), .. }) => s.value(),
        _ => panic!("Expected a string literal as the button description."),
    };

    let button_const = quote::format_ident!("BUTTON");

    let output = quote! {
        pub const #button_const: Button = Button {
            name: #fn_name_str,
            desc: #description,
            icon: include_str!(concat!("../../icons/", #fn_name_str, ".svg")),
            run: #fn_name,
        };

        #input_fn
    };

    output.into()
}