use codegen::{Scope, Struct, Type};
use openapi::v2::Schema;

use crate::helper::{clean_field_name, get_absolute_path_for_item, ref_to_absolute_path};
use crate::{CodegenDerive, DEFAULT_DERIVES, DEFAULT_SERDE_MACROS, scope_get_module_and_item_name_for_item};

pub fn generate_object(scope: &mut Scope, schema: &Schema, item_name: &str) {
    let structure_name = scope_get_module_and_item_name_for_item(scope, item_name).1;
    let mut structure = Struct::new(&structure_name);
    structure.derive_multiple(DEFAULT_DERIVES);
    structure.attribute(DEFAULT_SERDE_MACROS);

    if schema.properties.is_none() {
        println!("Object {} has no properties!", item_name);
        return;
    }

    for (name, property) in schema.properties.as_ref().unwrap() {
        if property.schema_type.is_none() {
            // it probably has ref_path set to another struct
            let ref_path = match property.ref_path.as_ref() {
                Some(ref_path) => ref_path,
                None => panic!("ref_path is not set for {}, but schema_type was empty!", name),
            };

            let name = clean_field_name(&name);
            let absolute_path = ref_to_absolute_path(ref_path);
            structure.field(&name, absolute_path).vis("pub");
            continue;
        }

        let schema_type = property.schema_type.as_ref().unwrap();
        dbg!(schema_type);
        let field_type = match schema_type.as_str() {
            "boolean" => Type::new("boolean"),
            "integer" => match property.format.as_ref() {
                Some(format) => match format.as_str() {
                    "int16" => Type::new("i16"),
                    "int32" => Type::new("i32"),
                    "int64" => Type::new("i64"),
                    "byte" => Type::new("u8"),
                    "uint16" => Type::new("u16"),
                    "uint32" => Type::new("u32"),
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

        structure.field(name.as_str(), field_type).vis("pub");
    }

    match scope_get_module_and_item_name_for_item(scope, item_name) {
        (Some(module), _) => { module.push_struct(structure); },
        (None, _) => { scope.push_struct(structure); },
    };
}