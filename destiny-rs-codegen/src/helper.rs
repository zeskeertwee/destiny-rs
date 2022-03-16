use codegen::Module;

pub trait CodegenDerive {
    fn _derive(&mut self, name: &str);

    fn derive_multiple(&mut self, names: &[&str]) {
        for name in names {
            self._derive(name);
        }
    }
}

impl CodegenDerive for codegen::Enum {
    fn _derive(&mut self, name: &str) {
        self.derive(name);
    }
}

impl CodegenDerive for codegen::Struct {
    fn _derive(&mut self, name: &str) {
        self.derive(name);
    }
}

pub fn clean_field_name(name: &str) -> String {
    match name {
        "type" => "r#type".to_string(),
        other => other.to_string(),
    }
}

pub fn scope_get_module_and_item_name_for_item<'a>(scope: &'a mut codegen::Scope, item_name: &str) -> (Option<&'a mut Module>, String) {
    let item_name = if item_name.starts_with(".") {
        format!("Extra{}", item_name)
    } else {
        item_name.to_string()
    };

    let mut modules: Vec<&str> = item_name.split('.').collect();
    let item_name = modules.pop().unwrap();
    if modules.len() == 0 {
        return (None, item_name.to_string());
    }

    let mut module = scope.get_or_new_module(modules[0]);
    for module_name in modules.iter().skip(1) {
        module = module.get_or_new_module(module_name);
    }
    module.vis("pub");

    module.import("serde", "{Deserialize, Serialize}");
    module.import("crate", "Hash");
    return (Some(module), item_name.to_string());
}

/// "Applications.ApplicationScopes" would become "crate::codegen::Applications::ApplicationScopes"
pub fn get_absolute_path_for_item(item_name: &str) -> String {
    let item_name = if item_name.starts_with(".") {
        format!("Extra{}", item_name)
    } else {
        item_name.to_string()
    };

    let mut modules: Vec<&str> = item_name.split('.').collect();
    let mut result = String::from("crate::codegen::");
    let last_item = modules.pop().unwrap();

    for module in modules {
        result.push_str(&format!("{}::", module));
    }
    result.push_str(last_item);

    return result;
}

pub fn ref_to_absolute_path(ref_name: &str) -> String {
    let clean_ref = ref_name.replace("#/definitions/", "");
    return get_absolute_path_for_item(&clean_ref);
}

pub fn dbg_print_generated(generated: &Vec<u8>) {
    let string = String::from_utf8(generated.clone()).unwrap();
    println!("{}", string);
}

pub fn dbg_print_statistics(generated: &Vec<u8>) {
    let string = String::from_utf8(generated.clone()).unwrap();
    let line_count = string.lines().count();
    let module_count = string.lines().filter(|line| line.contains("mod ")).count();
    let struct_count = string.lines().filter(|line| line.contains("struct ")).count();
    let enum_count = string.lines().filter(|line| line.contains("enum ")).count();

    println!("Generated {} lines", line_count);
    println!("Generated {} modules", module_count);
    println!("Generated {} structs", struct_count);
    println!("Generated {} enums", enum_count);
}