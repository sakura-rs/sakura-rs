use std::{
    collections::HashMap,
    fs::{self, read_to_string, File},
    io::{BufRead, BufReader},
    path::Path,
};

use quote::{quote, ToTokens};
use syn::{Field, Ident, Item, Type, TypePath};

pub fn main() {
    println!("cargo:rerun-if-changed=proto");

    const CLIENT_PROTO_FILE: &str = "proto/client.proto";
    const NORMAL_PROTO_FILE: &str = "proto/normalized.proto";

    let _ = fs::create_dir("gen/");
    if Path::new(CLIENT_PROTO_FILE).exists() {
        prost_build::Config::new()
            .out_dir("gen/")
            .type_attribute(".", "#[derive(sakura_proto_derive::CmdID)]")
            .compile_protos(&[CLIENT_PROTO_FILE], &["."])
            .unwrap();

        implement_cmd_id(Path::new("gen/client.rs")).unwrap();
    }

    if Path::new(NORMAL_PROTO_FILE).exists() {
        prost_build::Config::new()
            .out_dir("gen/")
            .type_attribute(".", "#[derive(sakura_proto_derive::CmdID)]")
            .compile_protos(&[NORMAL_PROTO_FILE], &["."])
            .unwrap();

        implement_cmd_id(Path::new("gen/normal.rs")).unwrap();
    }

    impl_proto_conversion(
        Path::new("gen/normal.rs"),
        Path::new("gen/client.rs"),
        Path::new("gen/client_to_normal.rs"),
        Path::new("gen/normal_to_client.rs"),
        Path::new("gen/conversion.rs"),
    )
    .unwrap();
}

#[must_use]
fn field_is_type(field: &Field, type_name: &str) -> bool {
    match &field.ty {
        Type::Path(TypePath { path, .. }) => {
            let last_segment = path.segments.last().unwrap();
            last_segment.ident == type_name
        }
        _ => panic!("Unsupported field type"),
    }
}

fn impl_proto_conversion(
    normal_path: &Path,
    client_path: &Path,
    c2n_out: &Path,
    n2c_out: &Path,
    conversion_out: &Path,
) -> std::io::Result<()> {
    let normal_file = read_to_string(normal_path)?;
    let client_file = read_to_string(client_path)?;

    let normal_file = syn::parse_file(&normal_file).unwrap();
    let client_file = syn::parse_file(&client_file).unwrap();

    let mut client_from_impls = quote! {};
    let mut normal_from_impls = quote! {};

    for item in client_file.items.iter() {
        match item {
            Item::Struct(client) => {
                let ident = &client.ident;
                let Some(Item::Struct(normal)) = normal_file.items.iter().find(|i| {
                    if let Item::Struct(s) = i {
                        s.ident == *ident
                    } else {
                        false
                    }
                }) else {
                    continue;
                };

                let mut assignments = quote! {};
                client
                    .fields
                    .iter()
                    .filter(|f| normal.fields.iter().any(|ff| ff.ident == f.ident))
                    .map(|f| (f, f.ident.as_ref().unwrap()))
                    .for_each(|(f, ident)| {
                        if field_is_type(f, "Option") {
                            assignments = quote! {
                                #assignments
                                #ident: value.#ident.map(|v| v.into()),
                            };
                        }
                        else if field_is_type(f, "Vec") {
                            assignments = quote! {
                                #assignments
                                #ident: value.#ident.into_iter().map(|v| v.into()).collect(),
                            };
                        }
                        else if field_is_type(f, "HashMap") {
                            assignments = quote! {
                                #assignments
                                #ident: value.#ident.into_iter().map(|(k, v)| (k.into(), v.into())).collect(),
                            };
                        }
                        else {
                            assignments = quote! {
                                #assignments
                                #ident: value.#ident.into(),
                            };
                        }
                    });

                client_from_impls = quote! {
                    #client_from_impls
                    impl From<crate::normal::#ident> for #ident {
                        fn from(value: crate::normal::#ident) -> Self {
                            Self {
                                #assignments
                                ..Default::default()
                            }
                        }
                    }
                };

                normal_from_impls = quote! {
                    #normal_from_impls
                    impl From<crate::client::#ident> for #ident {
                        fn from(value: crate::client::#ident) -> Self {
                            Self {
                                #assignments
                                ..Default::default()
                            }
                        }
                    }
                };
            }
            // oneof enums are placed in nested modules
            Item::Mod(nested_module) => {
                let ident = &nested_module.ident;

                // find same module in normal proto
                let Some(Item::Mod(normal_nested_module)) = normal_file.items.iter().find(|i| {
                    if let Item::Mod(m) = i {
                        m.ident == *ident
                    } else {
                        false
                    }
                }) else {
                    continue;
                };

                for item in nested_module.content.as_ref().unwrap().1.iter() {
                    if let Item::Enum(oneof_enum) = item {
                        let Some(Item::Enum(normal_oneof_enum)) = normal_nested_module
                            .content
                            .as_ref()
                            .unwrap()
                            .1
                            .iter()
                            .find(|i| {
                                if let Item::Enum(e) = i {
                                    e.ident == oneof_enum.ident
                                } else {
                                    false
                                }
                            })
                        else {
                            continue;
                        };

                        let mod_ident = &nested_module.ident;
                        let oneof_ident = &oneof_enum.ident;

                        let mut c2n_match_arms = quote! {};
                        let mut n2c_match_arms = quote! {};

                        oneof_enum
                            .variants
                            .iter()
                            .filter(|v| {
                                normal_oneof_enum
                                    .variants
                                    .iter()
                                    .any(|vv| vv.ident == v.ident)
                            })
                            .map(|v| &v.ident)
                            .for_each(|ident| {
                                n2c_match_arms = quote! {
                                    #n2c_match_arms
                                    crate::normal::#mod_ident::#oneof_ident::#ident(v) => Self::#ident(v.into()),
                                };

                                c2n_match_arms = quote! {
                                    #c2n_match_arms
                                    crate::client::#mod_ident::#oneof_ident::#ident(v) => Self::#ident(v.into()),
                                };
                            });

                        client_from_impls = quote! {
                            #client_from_impls
                            impl From<crate::normal::#mod_ident::#oneof_ident> for #mod_ident::#oneof_ident {
                                fn from(value: crate::normal::#mod_ident::#oneof_ident) -> Self {
                                    match value {
                                        #n2c_match_arms
                                        _ => unreachable!(),
                                    }
                                }
                            }
                        };

                        normal_from_impls = quote! {
                            #normal_from_impls
                            impl From<crate::client::#mod_ident::#oneof_ident> for #mod_ident::#oneof_ident {
                                fn from(value: crate::client::#mod_ident::#oneof_ident) -> Self {
                                    match value {
                                        #c2n_match_arms
                                        _ => unreachable!(),
                                    }
                                }
                            }
                        };
                    }
                }
            }
            &_ => (),
        };
    }

    let mut client_to_normal_packet = quote! {};
    let mut normal_to_client_packet = quote! {};

    for item in client_file.items.iter() {
        match item {
            Item::Struct(client) => {
                let ident = &client.ident;

                if !client.attrs.iter().any(|attr| {
                    attr.path()
                        .get_ident()
                        .map(|i| i == "cmdid")
                        .unwrap_or(false)
                }) {
                    continue;
                }

                if normal_file.items.iter().any(|i| {
                    if let Item::Struct(s) = i {
                        s.ident == *ident
                    } else {
                        false
                    }
                }) {
                    client_to_normal_packet = quote! {
                        #client_to_normal_packet
                        crate::client::#ident::CMD_ID => {
                            let proto = crate::client::#ident::decode(body)?;
                            let proto: crate::normal::#ident = proto.into();

                            Ok((crate::normal::#ident::CMD_ID, proto.encode_to_vec().into_boxed_slice()))
                        },
                    };

                    normal_to_client_packet = quote! {
                        #normal_to_client_packet
                        crate::normal::#ident::CMD_ID => {
                            let proto = crate::normal::#ident::decode(body)?;
                            let proto: crate::client::#ident = proto.into();

                            Ok((crate::client::#ident::CMD_ID, proto.encode_to_vec().into_boxed_slice()))
                        },
                    };
                }
            }
            &_ => (),
        }
    }

    let mut client_to_normal_combat_invocation = quote! {};
    let mut normal_to_client_combat_invocation = quote! {};

    let combat_invoke_names: HashMap<&'static str, &'static str> = HashMap::from([
        ("EntityMoveInfo", "EntityMove"),
        ("EvtBeingHitInfo", "EvtBeingHit"),
    ]);

    for item in client_file.items.iter() {
        match item {
            Item::Struct(client) => {
                let ident = &client.ident;
                if let Some(combat_enum_case) = combat_invoke_names.get(ident.to_string().as_str())
                {
                    let case_ident = Ident::new(&combat_enum_case, ident.span());
                    client_to_normal_combat_invocation.extend(quote! {
                        crate::normal::CombatTypeArgument::#case_ident => {
                            let proto = crate::client::#ident::decode(combat_data)?;
                            let proto: crate::normal::#ident = proto.into();

                            Ok(proto.encode_to_vec().into_boxed_slice())
                        }
                    });

                    normal_to_client_combat_invocation.extend(quote! {
                        crate::normal::CombatTypeArgument::#case_ident => {
                            let proto = crate::normal::#ident::decode(combat_data)?;
                            let proto: crate::client::#ident = proto.into();

                            Ok(proto.encode_to_vec().into_boxed_slice())
                        }
                    });
                }
            }
            &_ => (),
        }
    }

    let conversion_fn = quote! {
        #[allow(unused, warnings)] // disable lints for generated code
        pub fn client_to_normal(cmd_id: u16, body: &[u8]) -> Result<(u16, Box<[u8]>), ProtocolConversionError> {
            use crate::packet::{read_cmd_id, NetPacket};
            use crate::{Protobuf, CmdID};

            match cmd_id {
                #client_to_normal_packet
                not_found => Err(ProtocolConversionError::NotFound(not_found)),
            }
        }

        #[allow(unused, warnings)] // disable lints for generated code
        pub fn normal_to_client(cmd_id: u16, body: &[u8]) -> Result<(u16, Box<[u8]>), ProtocolConversionError> {
            use crate::packet::{read_cmd_id, NetPacket};
            use crate::{Protobuf, CmdID};

            match cmd_id {
                #normal_to_client_packet
                not_found => Err(ProtocolConversionError::NotFound(not_found)),
            }
        }

        #[allow(unused, warnings)] // disable lints for generated code
        pub fn combat_invocation_client_to_normal(arg_type: crate::normal::CombatTypeArgument, combat_data: &[u8]) -> Result<Box<[u8]>, ProtocolConversionError> {
            match arg_type {
                #client_to_normal_combat_invocation
                not_found => Err(ProtocolConversionError::NotFoundCombatArgument(arg_type)),
            }
        }
    };

    let client_from_impls_ast = syn::parse2(client_from_impls.into_token_stream()).unwrap();
    let normal_from_impls_ast = syn::parse2(normal_from_impls.into_token_stream()).unwrap();
    let conversion_fn_ast = syn::parse2(conversion_fn.into_token_stream()).unwrap();

    fs::write(n2c_out, prettyplease::unparse(&client_from_impls_ast))?;
    fs::write(c2n_out, prettyplease::unparse(&normal_from_impls_ast))?;
    fs::write(conversion_out, prettyplease::unparse(&conversion_fn_ast))?;

    Ok(())
}

fn implement_cmd_id(path: &Path) -> std::io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut output = Vec::new();

    let mut cmd_id_attr = None;
    for line in reader.lines() {
        let line = line?;
        let line_lower = line.to_lowercase();
        if line_lower.contains("cmdid: ") {
            cmd_id_attr = Some(make_cmd_id_attr(&line_lower).unwrap());
        } else {
            output.push(line);
            if let Some(attr) = cmd_id_attr.take() {
                output.push(attr);
            }
        }
    }

    fs::write(path, output.join("\n").as_bytes())?;
    Ok(())
}

fn make_cmd_id_attr(line: &str) -> Option<String> {
    let cmd_id = line.split("cmdid: ").nth(1)?.parse::<u16>().ok()?;
    Some(format!("#[cmdid({cmd_id})]"))
}
