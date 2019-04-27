#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

use im::Vector;
use rslisp::types::{BuiltinFunction, BuiltinMacro, Scope, Type};

use proc_macro::{TokenStream, TokenTree};
use quote::{quote, quote_spanned};

#[proc_macro_attribute]
pub fn builtin(attr: TokenStream, item: TokenStream) -> TokenStream {
    let builtin_type = match attr.into_iter().next() {
        Some(TokenTree::Ident(id)) => {
            let kind = id.to_string();

            if kind == "fn" {
                quote!(BuiltinFunction)
            } else if kind == "macro" {
                quote!(BuiltinMacro)
            } else {
                panic!("Attribute arguments may only be `macro` or `fn`");
            }
        }
        Some(t) => {
            t.span().error("Attribute must be an identifier").emit();
            panic!()
        }
        _ => panic!("Must have proper arguments"),
    };

    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let name = &input.ident;
    let name_str: String = input.ident.to_string();

    let block = &input.block;

    let concat = format!("{}_builtin", name);
    let name_builtin: syn::Ident = syn::Ident::new(&concat, name.span());

    let result = quote! {
        #input

        pub fn #name_builtin() -> #builtin_type {
            let fun = |args: Vector<Type>, scope: &mut Scope| -> Type {
                #block
            };

            #builtin_type::new(String::from(#name_str), fun)
        }
    };

    result.into()
}

// #[cfg(test)]
// mod test {
//     #[macro_use]
//     extern crate rslisp_derives;

//     // use rslisp_derives::builtin;
//     use rslisp::{Type, Scope};
//     use im::Vector;

//     #[builtin]
//     pub fn add(args: Vector<Type>, scope: &mut Scope) -> Type {
//         Type::Nil
//     }

//     #[test]
//     fn it_works() {
//         42;
//         ()
//     }
// }
