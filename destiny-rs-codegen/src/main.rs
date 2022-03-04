mod helper;

use std::error::Error;
use std::fs::File;
use std::io::Write;
use codegen;
use codegen::{Module, Scope};
use crate::helper::CodegenDerive;

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

            //for variant in schema.enum_values.as_ref().unwrap() {
            //    enumeration.new_variant(variant);
            //}

            for variant in schema.other.get("x-enum-values").unwrap().as_array().unwrap() {
                let variant_name = format!("{} = {}", variant["identifier"].as_str().unwrap(), variant["numericValue"].as_str().unwrap());
                let mut enum_variant = enumeration.new_variant(&variant_name);

                if let Some(desc) = variant["description"].as_str() {
                    enum_variant.doc(desc);
                }
            }

            enumeration.derive_multiple(DEFAULT_DERIVES);
            enumeration.attribute(DEFAULT_SERDE_MACROS);
        }
    }

    generated.write_all(scope.to_string().as_bytes())?;

    dbg_print_generated(&generated);

    Ok(())
}

fn scope_get_module_and_item_name_for_item<'a>(scope: &'a mut codegen::Scope, item_name: &str) -> (Option<&'a mut Module>, String) {
    let mut modules: Vec<&str> = item_name.split('.').collect();
    let item_name = modules.pop().unwrap();
    if modules.len() == 0 {
        return (None, item_name.to_string());
    }

    let mut module = scope.get_or_new_module(modules[0]);
    for module_name in modules.iter().skip(1) {
        module = module.get_or_new_module(module_name);
    }

    return (Some(module), item_name.to_string());
}

fn dbg_print_generated(generated: &Vec<u8>) {
    let string = String::from_utf8(generated.clone()).unwrap();
    println!("{}", string);
}