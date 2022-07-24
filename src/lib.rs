
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn sql(item: TokenStream) -> TokenStream {
    (&item.to_string().replace("--sql", "")).parse().unwrap()
}
