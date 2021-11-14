use proc_macro;
use proc_macro::bridge::server::TokenStream;

pub fn run() {

}

#[macro_export]
macro_rules! vec {
    ( $( $x: expr),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
/* this will result in
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
 */

// A procedural macro takes a TokenStream and outputs a Tokenstream
// #[some_attribute]
// pub fn some_name(input: TokenStream) -> TokenStream {}

// Check out the hello_macro and pancakes cargo project for procedural macros

/* Attribute macros:

What if we want to write something like this:

#[route(GET, "/")]
fn index() {

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {

We have two TokenStream's here. The first one represnets the GET, "/"
and the second one the function below




Function like macros can be used to have something like this:

let sql = sql!(SELECT * FROM posts WHERE id=1);

It would be implemented like this

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {

 */