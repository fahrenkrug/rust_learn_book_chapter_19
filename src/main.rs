mod unsafe_rust;
mod advanced_traits;
mod advanced_types;
mod advanced_functions_and_closures;
mod macros;

fn main() {
    unsafe_rust::run();
    advanced_traits::run();
    advanced_types::run();
    advanced_functions_and_closures::run();
    macros::run();
}

