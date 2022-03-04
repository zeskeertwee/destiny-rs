mod helper;
mod object;

use std::error::Error;
use std::fs::File;
use std::io::Write;
use codegen;
use codegen::{Import, Module, Scope, Type};
use crate::helper::{CodegenDerive, dbg_print_generated, dbg_print_statistics, ref_to_absolute_path, scope_get_module_and_item_name_for_item};

const CODEGEN_FILE_USES: &'static str = include_str!("./codegen_uses.rs");
const DEFAULT_DERIVES: &[&'static str] = &["Debug", "PartialEq", "Eq", "Hash", "Deserialize", "Serialize"];
const DEFAULT_SERDE_MACROS: &'static str = "#[serde(rename_all = \"camelCase\")]";

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("openapi-2.json")?;
    let spec = openapi::from_reader(file).unwrap();


    let mut generated = Vec::new();
    generated.write_all(CODEGEN_FILE_USES.as_bytes())?;

    // base API path
    let base_api_path = format!(r#"const BASE_API_PATH: &'static str = "{}{}";"#, spec.host.unwrap(), spec.base_path.unwrap());
    generated.write_all(base_api_path.as_bytes())?;

    let mut scope = Scope::new();

    for (name, schema) in spec.definitions.unwrap().iter() {
        if schema.enum_values.is_some() {
            // it's an enum
            let (module, item_name) = scope_get_module_and_item_name_for_item(&mut scope, name);

            let enumeration = match module {
                Some(module) => module.new_enum(&item_name),
                None => scope.new_enum(&item_name),
            };

            let format = match &schema.format {
                Some(format) => match format.as_str() {
                    "byte" => "u8",
                    "int32" => "i32",
                    "int64" => "i64",
                    other => panic!("unsupported format: {}", other),
                },
                None => panic!("no format specified for enum: {}", name),
            };

            enumeration.repr(format);

            for variant in schema.other.get("x-enum-values").unwrap().as_array().unwrap() {
                let identifier = variant["identifier"].as_str().unwrap();
                let numeric_value = variant["numericValue"].as_str().unwrap();
                let variant_name = format!("{} = {}", identifier, numeric_value);
                let mut enum_variant = enumeration.new_variant(&variant_name).annotation(&format!("#[serde(rename = \"{}\")]", numeric_value));

                if let Some(desc) = variant["description"].as_str() {
                    enum_variant.doc(desc);
                }
            }

            enumeration.derive_multiple(DEFAULT_DERIVES);
            enumeration.attribute(DEFAULT_SERDE_MACROS);
            continue;
        }

        if let Some(schema_type) = &schema.schema_type {
            dbg!(name, &schema);
            match schema_type.as_str() {
                "object" => {
                    object::generate_object(&mut scope, &schema, &name);

                    //for (name, property) in schema.properties.as_ref().unwrap().iter() {
                    //    if property.schema_type.is_none() {
                    //        // it probably has ref_path set to another struct
                    //        structure.field(&name, Type::new(&ref_to_absolute_path(&property.ref_path.as_ref().unwrap())));
                    //        continue;
                    //    }
                    //
                    //    let field_type = Type::new(&match property.schema_type.as_ref().unwrap().as_str() {
                    //        "string" => "String".to_string(),
                    //        "integer" => "i64".to_string(),
                    //        "boolean" => "bool".to_string(),
                    //        "number" => match property.format.as_ref().unwrap().as_str() {
                    //            "float" => "f64".to_string(),
                    //            "double" => "f64".to_string(),
                    //            _ => panic!("Unsupported number format: {}", property.format.as_ref().unwrap().as_str()),
                    //        },
                    //        "array" => {
                    //            format!("Vec<{}>", ref_to_absolute_path(&property.items.as_ref().unwrap().ref_path.as_ref().unwrap()))
                    //        },
                    //        "object" => {
                    //            println!("TODO: object!");
                    //            continue;
                    //        },
                    //        _ => panic!("Unsupported type: {}", property.schema_type.as_ref().unwrap()),
                    //    });
                    //
                    //    structure.field(&name, field_type);
                    //}
                },
                "integer" => {
                    if schema.enum_values.is_none() {
                        panic!("Integer schema type without enum!");
                    }

                    // we already generated the enum, so we can ignore this
                },
                "array" => {
                    println!("Skipping array type: {}", name);
                }
                _ => panic!("Unknown schema type: {}", schema_type),
            }
        }
    }

    generated.write_all(scope.to_string().as_bytes())?;

    dbg_print_generated(&generated);
    println!();
    dbg_print_statistics(&generated);

    if let Ok(val) = std::env::var("OUTPUT_TO_FILE") {
        println!("Writing to file: {}", val);
        let mut file = std::fs::File::create(val)?;
        file.write_all(&generated)?;
    }

    Ok(())
}
