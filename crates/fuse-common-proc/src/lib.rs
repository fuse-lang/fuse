use quote::{quote, ToTokens};

#[proc_macro_attribute]
pub fn serializable(
    _: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    quote! {
        #[cfg_attr(any(test, feature = "serde"), derive(serde::Serialize, serde::Deserialize))]
        #input
    }
    .into_token_stream()
    .into()
}
