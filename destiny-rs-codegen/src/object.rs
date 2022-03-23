use std::collections::BTreeMap;
use codegen::{Scope, Struct, Type};
use openapi::v2::Schema;

use crate::helper::{clean_field_name, get_absolute_path_for_item, is_hash_blacklisted, ref_to_absolute_path};
use crate::{CodegenDerive, DEFAULT_DERIVES, DEFAULT_SERDE_MACROS, HASH_BLACKLIST, scope_get_module_and_item_name_for_item};
use crate::endpoint::additional_properties_workaround;

pub fn generate_object(scope: &mut Scope, schema: &Schema, item_name: &str) {
    let structure_name = scope_get_module_and_item_name_for_item(scope, item_name).1;
    let mut structure = Struct::new(&structure_name);
    structure.derive_multiple(DEFAULT_DERIVES);
    structure.attribute(DEFAULT_SERDE_MACROS);
    let mut schema = schema.to_owned();

    if schema.properties.is_none() {
        println!("Object {} has no properties!", item_name);

        //schema = additional_properties_workaround(&schema);

        return;
    }

    for (name, property) in schema.properties.as_ref().unwrap() {
        if property.schema_type.is_none() {
            // it probably has ref_path set to another struct
            let ref_path = match property.ref_path.as_ref() {
                Some(ref_path) => ref_path,
                None => panic!("ref_path is not set for {}, but schema_type was empty!", name),
            };

            let absolute_path = ref_to_absolute_path(ref_path);
            structure.field(&clean_field_name(&name), absolute_path).vis("pub");
            continue;
        }

        let schema_type = property.schema_type.as_ref().unwrap();
        //dbg!(schema_type);
        let field_type = match schema_type.as_str() {
            "boolean" => Type::new("bool"),
            "integer" => match property.format.as_ref() {
                Some(format) => match format.as_str() {
                    "int16" => Type::new("i16"),
                    "int32" => Type::new("i32"),
                    "int64" => Type::new("i64"),
                    "byte" => Type::new("u8"),
                    "uint16" => Type::new("u16"),
                    "uint32" => {
                        // could be a hash
                        let mut mapped_def = property.other.get("x-mapped-definition");

                        if let Some(def) = mapped_def {
                            if is_hash_blacklisted(def["$ref"].as_str().unwrap()) {
                                println!("Blacklisted hash: {}", item_name);
                                mapped_def = None;
                            } else {
                                println!("Non-blacklisted hash: {}", item_name);
                            }
                        }

                        match mapped_def {
                            Some(def) => Type::new(&format!("Hash<{}>", ref_to_absolute_path(def["$ref"].as_str().unwrap()))),
                            None => Type::new("u32"),
                        }
                    },
                    "uint64" => Type::new("u64"),
                    _ => panic!("Unsupported format {} for integer!", format),
                },
                None => panic!("Format is not set for type integer!"),
            },
            "number" => match property.format.as_ref() {
                Some(format) => match format.as_str() {
                    "float" => Type::new("f32"),
                    "double" => Type::new("f64"),
                    _ => panic!("Unsupported format {} for number!", format),
                },
                None => panic!("Format is not set for type number!"),
            },
            "string" => {
                // TODO: datetime format
                Type::new("String")
            },
            "array" => {
                if property.items.is_none() {
                    dbg!(&schema);
                    panic!("Array {} has no items!", name);
                }

                let item_schema = property.items.as_ref().unwrap();
                if item_schema.ref_path.is_none() {
                    // we probabbly have an array of hashes
                    if item_schema.schema_type.is_none() {
                        dbg!(item_schema);
                        panic!("Array {} has no type/format!", name);
                    }

                    let item_type = match item_schema.schema_type.as_ref().unwrap().as_str() {
                        "integer" => match item_schema.format.as_ref() {
                            Some(format) => match format.as_str() {
                                "int16" => "i16",
                                "int32" => "i32",
                                "int64" => "i64",
                                "uint32" => "u32",
                                "uint64" => "u64",
                                _ => panic!("Unsupported format {} for integer!", format),
                            },
                            None => panic!("Format is not set for type integer!"),
                        },
                        "boolean" => "bool",
                        "string" => "String",
                        other => panic!("Unsupported format {} for array!", other),
                    };

                    Type::new(&format!("Vec<{}>", item_type))
                } else {
                    let absolute_path = ref_to_absolute_path(item_schema.ref_path.as_ref().unwrap());
                    Type::new(&format!("Vec<{}>", absolute_path))
                }
            },
            "object" => {
                generate_object(scope, property, name);
                Type::new(&format!("{}", get_absolute_path_for_item(name)))
            },
            _ => panic!("Unsupported type {}!", schema_type),
        };

        structure.field(&clean_field_name(name.as_str()), field_type).vis("pub");
    }

    if structure_name.starts_with("Destiny")
        && structure_name.ends_with("Definition")
        && schema.other.contains_key("x-mobile-manifest-name")
    {
        // it's mapped to a manifest table, since all manifest table structs follow the same naming convention:
        // for example: DestinyInventoryItemDefinition

        let mut enum_var_name = structure_name.to_string();
        enum_var_name = enum_var_name.replace("Destiny", "");
        enum_var_name = enum_var_name.replace("Definition", "");

        let mut manifest_key_impl = codegen::Impl::new(structure.ty());
            manifest_key_impl.impl_trait("crate::traits::manifest_key::ManifestTableKey")
            .new_const("TABLE_KEY",
                       Type::new("crate::models::manifest::ManifestKey"),
                       Type::new(&format!("crate::models::manifest::ManifestKey::{}", enum_var_name)));

        let module = scope_get_module_and_item_name_for_item(scope, item_name).0;

        match module {
            Some(module) => { module.push_impl(manifest_key_impl); },
            None => { scope.push_impl(manifest_key_impl); },
        }
    }

    match scope_get_module_and_item_name_for_item(scope, item_name) {
        (Some(module), _) => { module.push_struct(structure); },
        (None, _) => { scope.push_struct(structure); },
    };
}