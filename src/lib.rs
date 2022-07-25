
extern crate proc_macro;
use proc_macro::TokenStream;

// fix inline syntax highlight on vscode as execute() does not allow comments
#[proc_macro]
pub fn sql(item: TokenStream) -> TokenStream {
    (&item.to_string().replace("--sql", "")).parse().unwrap()
}
