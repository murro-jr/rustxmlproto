#![recursion_limit = "256"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

fn impl_proto_deserialize(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let param = get_vector_input(&name.to_string());

    quote! {
        impl<'de> serde::Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct DerivedVisitor;

                impl<'d> serde::de::Visitor<'d> for DerivedVisitor {
                    type Value = Vec<#param>;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                        let expect_string = format!("Expecting a map of values");
                        f.write_str(&expect_string)
                    }

                    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
                    where
                        M: serde::de::MapAccess<'d>,
                    {
                        let mut params: Self::Value = Vec::new();

                        while let Some((key, mut value)) = access.next_entry::<String, #param>()? {
                            if key != "generic" {
                                value.datatype = Some(key);
                            } else {
                                value.datatype = None;
                            }
                            params.push(value);
                        }

                        Ok(params)
                    }
                }

                Ok(#name(deserializer.deserialize_map(DerivedVisitor)?))
            }
        }

        impl Default for #name {
            fn default() -> Self {
                #name(Vec::new())
            }
        }
    }
}

const PARAMETERS: &str = "Parameters";
const FUNCTIONS: &str = "Functions";
const MEMBERS: &str = "Members";

fn get_vector_input(class: &String) -> syn::Ident {
    match &class[..] {
        PARAMETERS => syn::Ident::from("Parameter"),
        FUNCTIONS => syn::Ident::from("Function"),
        MEMBERS => syn::Ident::from("Member"),
        _ => syn::Ident::from(""),
    }
}

#[proc_macro_derive(ProtoDeserializer, attributes(input))]
pub fn proto_deserializer(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_proto_deserialize(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}
