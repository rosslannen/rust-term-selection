use proc_macro2::TokenStream;

use quote::quote;

use syn::{parse_macro_input, Attribute, Data, DeriveInput, Lit, LitStr, Meta, MetaNameValue};

#[proc_macro_derive(TermSelection, attributes(prompt, description))]
pub fn term_selection(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the tokens.
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;

    let prompt_impl = impl_prompt(&input);

    let options_impl = impl_options(&input);

    let description_impl = impl_description(&input);

    let expanded = quote! {
        impl ::term_selection::TermSelection for #ident {
            #prompt_impl

            #description_impl

            #options_impl
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn impl_prompt(input: &DeriveInput) -> TokenStream {
    // Find the prompt attribute. Looks like `#[prompt = "prompt"]`.
    let prompt = find_attr_str_lit(&input.attrs, "prompt").unwrap_or({
        // If it is not there, fall back onto the name of the struct.
        let ident = &input.ident;
        LitStr::new(&format!("{}", ident), ident.span())
    });

    quote! {
        fn prompt() -> &'static str {
            #prompt
        }
    }
}

fn impl_description(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;

    let enum_data = match &input.data {
        Data::Enum(data) => data,
        _ => unimplemented!(),
    };
    let descriptions = enum_data
        .variants
        .iter()
        .map(|variant| {
            let ident = &variant.ident;
            let description = find_attr_str_lit(&variant.attrs, "description")
                .unwrap_or(LitStr::new(&format!("{}", ident), ident.span()));

            quote! { #name::#ident => #description, }
        })
        .collect::<Vec<_>>();

    quote! {
        fn description(self) -> &'static str {
            match self {
                #(#descriptions)*
            }
        }
    }
}

fn impl_options(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;

    let enum_data = match &input.data {
        Data::Enum(data) => data,
        _ => unimplemented!(),
    };
    let options = enum_data
        .variants
        .iter()
        .map(|variant| {
            let ident = &variant.ident;

            quote! { #name::#ident }
        })
        .collect::<Vec<_>>();

    quote! {
        fn options() -> &'static [Self] {
            &[#(#options),*]
        }
    }
}

fn find_attr_str_lit<'a, A>(attrs: A, name: &str) -> Option<LitStr>
where
    A: IntoIterator<Item = &'a Attribute>,
{
    attrs
        .into_iter()
        .find_map(|attr| match attr.parse_meta().ok() {
            Some(Meta::NameValue(MetaNameValue { path, lit, .. })) => match (path.get_ident(), lit)
            {
                (Some(ident), Lit::Str(lit_str)) if ident == name => Some(lit_str),
                _ => None,
            },
            _ => None,
        })
}
