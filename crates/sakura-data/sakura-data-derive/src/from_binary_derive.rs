use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, Type, TypePath};

const TYPE_OPTION: &str = "Option";

#[must_use]
fn field_is_optional(field: &Field) -> bool {
    match &field.ty {
        Type::Path(TypePath { path, .. }) => {
            let last_segment = path.segments.last().unwrap();
            last_segment.ident == TYPE_OPTION
        }
        _ => panic!("Unsupported field type"),
    }
}

pub fn impl_from_binary(input: DeriveInput) -> TokenStream {
    let ident = input.ident;

    let mut assignments = quote! {};

    match input.data {
        Data::Struct(data) => {
            let Fields::Named(fields) = data.fields else {
                panic!("#[derive(FromBinary)] only supports structs with named fields");
            };

            for (idx, field) in fields.named.iter().enumerate() {
                let field_name = field.ident.as_ref().unwrap();

                if field_is_optional(field) {
                    assignments.extend(quote! {
                        #field_name: if exist_flag.exists(#idx) {
                            Some(crate::FromBinary::from_binary(r)?)
                        }
                        else {
                            None
                        },
                    });
                } else {
                    assignments.extend(quote! {
                        #field_name: if exist_flag.exists(#idx) {
                            crate::FromBinary::from_binary(r)?
                        }
                        else {
                            Default::default()
                        },
                    });
                }
            }

            quote! {
                impl crate::FromBinary for #ident {
                    fn from_binary<R: ::std::io::Read + ::std::io::Seek>(r: &mut R) -> ::std::io::Result<Self> {
                        use byteorder::ReadBytesExt;
                        use varint_rs::VarintReader;

                        let exist_flag: crate::util::ExistFlag = crate::FromBinary::from_binary(r)?;

                        Ok(Self {
                            #assignments
                        })
                    }
                }
            }
        }
        Data::Enum(data) => {
            let mut match_arms = quote! {};

            for var in data.variants.iter() {
                let name = &var.ident;
                let (_, discriminant) = var.discriminant.as_ref().unwrap();
                match_arms.extend(quote! {
                    #discriminant => Ok(Self::#name),
                });
            }

            quote! {
                impl crate::FromBinary for #ident {
                    fn from_binary<R: ::std::io::Read + ::std::io::Seek>(r: &mut R) -> ::std::io::Result<Self> {
                        match i32::from_binary(r)? {
                            #match_arms
                            unk => panic!("{}::from_binary: unknown variant: {unk}", stringify!(#ident)),
                        }
                    }
                }
            }
        }
        Data::Union(_) => panic!("Union types are not supported by from_binary_derive"),
    }
}
