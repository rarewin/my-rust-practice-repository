use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

#[derive(HelloMacro)]
struct Onigiri;

fn main() {
    Pancakes::hello_macro();
    Onigiri::hello_macro();
}
