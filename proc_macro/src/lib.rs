//#![feature(proc_macro)]

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;
extern crate proc_macro2;

use proc_macro2::TokenStream;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn command(attribute: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    //println!("A: {:?}", &attribute);
    //println!("I: {:#?}", &input);
    let original = input.clone();
    let struct_def = parse_macro_input!(original as ItemStruct);
    let struct_name = &struct_def.ident;
    let input: TokenStream = input.into();
    let output = quote! {
        #input

        impl Command for #struct_name {
        }
    };
    output.into()
    // Solution: To make the attribute on fields work, read the attribute and remove it afterward
    // from the field, otherwise the compiler searches for its meaning again.

    // actually do some modification to the struct and also read field annotated with `#[prop]`
    // to create prop struct
}
