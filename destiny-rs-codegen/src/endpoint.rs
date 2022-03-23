use codegen::{Block, Scope, Type};
use openapi::v2::{ParameterOrRef, Schema};
use url_builder::URLBuilder;
use crate::helper::{clean_field_name, get_absolute_path_for_item};
use crate::object::generate_object;
use crate::scope_get_module_and_item_name_for_item;

pub fn generate_endpoints(scope: &mut Scope, spec: &openapi::v2::Spec) {
    println!("Generating endpoints");
    for (path, path_item) in spec.paths.iter() {
        let route = match path_item.get.as_ref() {
            Some(get) => get,
            None => {
                match path_item.post.as_ref() {
                    // not yet implemented
                    Some(post) => continue,
                    None => panic!("No get or post for path: {}", path),
                }
            }
        };

        let operation_id = route.operation_id.as_ref().unwrap();
        let function = match scope_get_module_and_item_name_for_item(scope, operation_id) {
            (Some(module), name) => module.new_fn(&name),
            (None, name) => scope.new_fn(&name),
        };
        function.vis("pub");
        function.set_async(true);
        function.arg("client", Type::new("&reqwest::Client"));

        let response_item_name = format!("{}Response", operation_id);
        let response_absolute_path = get_absolute_path_for_item(&response_item_name);

        function.ret(Type::new(&format!("anyhow::Result<{}>", response_absolute_path)));
        let mut function_params = Vec::new();

        match &route.parameters {
            Some(parameters) => {
                println!("{}", parameters.len());
                for parameter in parameters {
                    match parameter {
                        ParameterOrRef::Ref { ref_path } => panic!("Path parameter reference not supported! (ref: {})", ref_path),
                        ParameterOrRef::Parameter { name, location, required, description, param_type, format, .. } => {
                            // every parameter has a description
                            dbg!(path, name, location, required, description, param_type, format);
                            let description = description.as_ref().unwrap();
                            let param_type = match param_type.as_ref().unwrap().as_str() {
                                "boolean" => "bool",
                                "integer" => match format.as_ref() {
                                    Some(format) => match format.as_str() {
                                            "int16" => "i16",
                                            "int32" => "i32",
                                            "int64" => "i64",
                                            "byte" => "u8",
                                            "uint16" => "u16",
                                            "uint32" => "u32",
                                            "uint64" => "u64",
                                            x => panic!("Unsupported format {} for integer!", x),
                                        },
                                    None => panic!("Format is not set for type integer!"),
                                    },

                                "number" => match format.as_ref() {
                                    Some(format) => match format.as_str() {
                                        "float" => "f32",
                                        "double" => "f64",
                                        x => panic!("Unsupported format {} for number!", x),
                                    },
                                    None => panic!("Format is not set for type number!"),
                                },
                                "string" => {
                                    // TODO: datetime format
                                    "String"
                                },
                                _ => "()"
                                // TODO: fix this
                                //x => panic!("Unsupported type {}", x),
                            };

                            // it appears that this is only set if it's required
                            let required = required.unwrap_or(false);

                            function_params.push(match required {
                                true => (name.to_string(), Type::new(param_type)),
                                false => (name.to_string(), Type::new(&format!("Option<{}>", param_type))),
                            });

                            println!("{}: {}", name, description);
                        },
                    }
                }

                for (param_name, param_type) in function_params {
                    function.arg(&clean_field_name(&param_name), param_type);
                }

                let mut url = URLBuilder::new();
                url.set_protocol("https");
                url.set_host("bungie.net");
                url.push_route("/Platform");
                url.push_route(&path);

                function.line(&format!(r#"let url = format!("{}");"#, url.build()));
                function.line(&format!("Ok(crate::codegen::get_request(client, &url).await?)"));
            },
            None => (),
        }

        //let response_struct = match scope_get_module_and_item_name_for_item(scope, &response_item_name) {
        //    (Some(module), name) => module.new_struct(&name),
        //    (None, name) => scope.new_struct(&name),
        //};

        match route.responses.get("200") {
            Some(response) => {
                println!("Gen response for {}", operation_id);
                generate_object(scope, &response.schema.as_ref().unwrap(), &response_item_name);
            }
            None => panic!("{} has no 200 response!", operation_id),
        }
    }
}


// example:
//"MessageData": Schema {
//  ref_path: None,
//  description: None,
//  schema_type: Some(
//      "object",
//  ),
//  format: None,
//  enum_values: None,
//  required: None,
//  items: None,
//  properties: None,
//  all_of: None,
//  other: {
//      "additionalProperties": Object({
//          -- THIS HERE CAN ALSO BE $ref --
//          "type": String(
//              "string",
//          ),
//      }),
//  },
//},
pub fn additional_properties_workaround(schema: &Schema) -> Schema {
    let mut schema = schema.to_owned();
    println!("Trying workaround..");

    if schema.properties.is_some() {
        // not necessary
        println!("Properties not empty!");
        return schema;
    }

    if let Some(additional_properties) = schema.other.get("additionalProperties") {
        match additional_properties["type"].as_str() {
            Some(field_type) => {
                println!("Overwrote type of schema to {}", field_type);
                schema.schema_type = Some(field_type.to_owned());
            },
            None => panic!("No type!"),
        };
    } else {
        panic!("No additional properties!");
    }

    schema
}