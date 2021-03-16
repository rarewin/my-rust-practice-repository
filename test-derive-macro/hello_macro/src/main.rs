// use crate::HelloMacro;
use hello_macro_derive::HelloMacro;

pub trait HelloMacro {
    fn hello_macro();
}

#[derive(HelloMacro)]
struct Pancakes;

#[derive(HelloMacro)]
struct Onigiri;

fn main() {
    Pancakes::hello_macro();
    Onigiri::hello_macro();
}
