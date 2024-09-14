use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parser, parse_macro_input, spanned::Spanned, Expr, ItemFn, Lit};

#[proc_macro_attribute]
pub fn my_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = syn::punctuated::Punctuated::<syn::MetaNameValue, syn::Token![,]>::parse_terminated
        .parse(attr)
        .unwrap();

    let mut name_opt = None;
    for arg in args {
        let ident = arg.path.get_ident().span().source_text().unwrap();
        if ident == "name" {
            if let Expr::Lit(lit) = arg.value {
                if let Lit::Str(s) = lit.lit {
                    name_opt = Some(s);
                }
            }
        }
    }

    let input_fn = parse_macro_input!(item as ItemFn);

    let ident = &input_fn.sig.ident;

    let name = name_opt.unwrap();

    let output = quote! {
        #input_fn

        inventory::submit!(Test{
            name: #name,
            f: #ident
       });
    };

    output.into()
}
