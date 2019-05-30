//#![feature(proc_macro)]

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;
extern crate proc_macro2;

use proc_macro2::TokenStream;

#[proc_macro_attribute]
pub fn command(attribute: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    //println!("A: {:?}", &attribute);
    //println!("I: {:#?}", &input);
    let input: TokenStream = input.into();
    //println!("I: {:#?}", &input);
    let output = quote! {
        #input
    };
    output.into()
    // Solution: To make the attribute on fields work, read the attribute and remove it afterward
    // from the field, otherwise the compiler searches for its meaning again.

    // actually do some modification to the struct and also read field annotated with `#[prop]`
    // to create prop struct
}
