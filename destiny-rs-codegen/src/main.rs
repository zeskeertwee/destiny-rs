#![allow(clippy::enum_variant_names)]

use lazy_static::lazy_static;
use regex::Regex;

fn camel_case_to_snake_case(s: &str) -> String {
    let mut words = Vec::new();
    let mut current_word_buffer = String::new();
    for ch in s.chars() {
        if ch.is_uppercase() {
            if !current_word_buffer.is_empty() {
                words.push(current_word_buffer.clone());
                current_word_buffer.clear();
            }
            current_word_buffer.push(ch.to_ascii_lowercase());
        } else {
            current_word_buffer.push(ch);
        }
    }
    words.push(current_word_buffer);
    words.join("_")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Component {
    Profiles,
    VendorReceipts,
    ProfileInventories,
    ProfileCurrencies,
    ProfileProgression,
    PlatformSilver,
    Characters,
    CharacterInventories,
    CharacterProgressions,
    CharacterRenderData,
    CharacterActivities,
    CharacterEquipment,
    ItemInstances,
    ItemObjectives,
    ItemPerks,
    ItemRenderData,
    ItemStats,
    ItemSockets,
    ItemTalentGrids,
    ItemCommonData,
    ItemPlugStates,
    ItemPlugObjectives,
    ItemReusablePlugs,
    Vendors,
    VendorCategories,
    VendorSales,
    Kiosks,
    CurrencyLookups,
    PresentationNodes,
    Collectibles,
    Records,
    Transitory,
    Metrics,
    StringVariables,
    Craftables,
}

impl Component {
    fn new(s: &str) -> Self {
        match s {
            "Profiles" => Self::Profiles,
            "VendorReceipts" => Self::VendorReceipts,
            "ProfileInventories" => Self::ProfileInventories,
            "ProfileCurrencies" => Self::ProfileCurrencies,
            "ProfileProgression" => Self::ProfileProgression,
            "PlatformSilver" => Self::PlatformSilver,
            "Characters" => Self::Characters,
            "CharacterInventories" => Self::CharacterInventories,
            "CharacterProgressions" => Self::CharacterProgressions,
            "CharacterRenderData" => Self::CharacterRenderData,
            "CharacterActivities" => Self::CharacterActivities,
            "CharacterEquipment" => Self::CharacterEquipment,
            "ItemInstances" => Self::ItemInstances,
            "ItemObjectives" => Self::ItemObjectives,
            "ItemPerks" => Self::ItemPerks,
            "ItemRenderData" => Self::ItemRenderData,
            "ItemStats" => Self::ItemStats,
            "ItemSockets" => Self::ItemSockets,
            "ItemTalentGrids" => Self::ItemTalentGrids,
            "ItemCommonData" => Self::ItemCommonData,
            "ItemPlugStates" => Self::ItemPlugStates,
            "ItemPlugObjectives" => Self::ItemPlugObjectives,
            "ItemReusablePlugs" => Self::ItemReusablePlugs,
            "Vendors" => Self::Vendors,
            "VendorCategories" => Self::VendorCategories,
            "VendorSales" => Self::VendorSales,
            "Kiosks" => Self::Kiosks,
            "CurrencyLookups" => Self::CurrencyLookups,
            "PresentationNodes" => Self::PresentationNodes,
            "Collectibles" => Self::Collectibles,
            "Records" => Self::Records,
            "Transitory" => Self::Transitory,
            "Metrics" => Self::Metrics,
            "StringVariables" => Self::StringVariables,
            "Craftables" => Self::Craftables,
            _ => panic!("Invalid component: {}", s),
        }
    }
    fn as_string(&self) -> String {
        match self {
            Component::Profiles => "Profiles",
            Component::VendorReceipts => "VendorReceipts",
            Component::ProfileInventories => "ProfileInventories",
            Component::ProfileCurrencies => "ProfileCurrencies",
            Component::ProfileProgression => "ProfileProgression",
            Component::PlatformSilver => "PlatformSilver",
            Component::Characters => "Characters",
            Component::CharacterInventories => "CharacterInventories",
            Component::CharacterProgressions => "CharacterProgressions",
            Component::CharacterRenderData => "CharacterRenderData",
            Component::CharacterActivities => "CharacterActivities",
            Component::CharacterEquipment => "CharacterEquipment",
            Component::ItemInstances => "ItemInstances",
            Component::ItemObjectives => "ItemObjectives",
            Component::ItemPerks => "ItemPerks",
            Component::ItemRenderData => "ItemRenderData",
            Component::ItemStats => "ItemStats",
            Component::ItemSockets => "ItemSockets",
            Component::ItemTalentGrids => "ItemTalentGrids",
            Component::ItemCommonData => "ItemCommonData",
            Component::ItemPlugStates => "ItemPlugStates",
            Component::ItemPlugObjectives => "ItemPlugObjectives",
            Component::ItemReusablePlugs => "ItemReusablePlugs",
            Component::Vendors => "Vendors",
            Component::VendorCategories => "VendorCategories",
            Component::VendorSales => "VendorSales",
            Component::Kiosks => "Kiosks",
            Component::CurrencyLookups => "CurrencyLookups",
            Component::PresentationNodes => "PresentationNodes",
            Component::Collectibles => "Collectibles",
            Component::Records => "Records",
            Component::Transitory => "Transitory",
            Component::Metrics => "Metrics",
            Component::StringVariables => "StringVariables",
            Component::Craftables => "Craftables",
        }
        .to_string()
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Attribute {
    Enumeration,
    Nullable,
    MobileManifestEntity,
    DependsOn(Component),
    Mapped(String),
}

#[derive(Debug)]
enum EnumerationType {
    Byte,
    Int32,
    Int64,
}

impl EnumerationType {
    fn from_str(s: &str) -> EnumerationType {
        match s {
            "byte" => EnumerationType::Byte,
            "int32" => EnumerationType::Int32,
            "int64" => EnumerationType::Int64,
            _ => panic!("Invalid EnumerationType: {}", s),
        }
    }
}

#[derive(Debug)]
struct EnumerationValue {
    name: String,
    value: String,
    description: Option<String>,
}

#[derive(Debug)]
struct Enumeration {
    id: String,
    enumeration_type: EnumerationType,
    description: Option<String>,
    values: Vec<EnumerationValue>,
}

impl Enumeration {
    fn new(s: &str) -> Enumeration {
        lazy_static! {
            static ref RE1: Regex =
                Regex::new(r#"<a id="[^"]+?"><h3>[A-Za-z\.0-9]+?</h3></a>"#).unwrap();
            static ref RE2: Regex = Regex::new(r#"<h3>[A-Za-z\.0-9]+?</h3>"#).unwrap();
            static ref RE3: Regex =
                Regex::new(r#"<strong>Type</strong>: [a-z\-]+(16|32|64)?"#).unwrap();
            static ref RE4: Regex = Regex::new(r#"<div class="description">[\r\s]+?.+?[\r\s]+?</div>"#).unwrap();
            static ref RE5: Regex = Regex::new(r#"<li>[\r\s]+?<div><strong>.+?</strong>: \d+?</div>([\r\s]+?<div class="description">.+?</div>)?[\r\s]+?</li>"#).unwrap();
            static ref RE6: Regex = Regex::new(r#"<div class="description">.+?</div>"#).unwrap();
            static ref RE7: Regex = Regex::new(r#"<div><strong>[a-zA-Z0-9]+?</strong>: \d+?</div>"#).unwrap();
            static ref RE8: Regex = Regex::new(r#"<strong>[a-zA-Z0-9]+?</strong>"#).unwrap();
            static ref RE9: Regex = Regex::new(r#"</strong>: \d+?</div>"#).unwrap();
        }

        let id = {
            let id = &RE1.captures(s).unwrap()[0];
            let id = &RE2.captures(id).unwrap()[0];
            id[4..(id.len() - 5)].to_string()
        };

        let enumeration_type = {
            let cap = &RE3.captures(s).unwrap()[0];
            //println!("RE3 cap: {}", cap);
            EnumerationType::from_str(cap.split(':').last().unwrap().trim())
        };

        let description = RE4.captures(s).map(|c| c[0].to_string());

        let values = RE5
            .captures_iter(s)
            .map(|cap| {
                let cap = &cap[0];
                {
                    let (name, value) = {
                        let name_value = &RE7
                            .captures(cap)
                            .unwrap_or_else(|| panic!("Failed RE7 Capture: {}", cap))[0];
                        let name = &RE8.captures(name_value).unwrap()[0];
                        let name = name[8..(name.len() - 9)].to_string();

                        let value = &RE9.captures(name_value).unwrap()[0];
                        let value = value[11..(value.len() - 6)].to_string();

                        (name, value)
                    };

                    let description = if cap.contains("<div class=\"description\">") {
                        let description = &RE6.captures(cap).unwrap()[0];
                        Some(description[25..(description.len() - 6)].to_string())
                    } else {
                        None
                    };

                    EnumerationValue {
                        name,
                        value,
                        description,
                    }
                }
            })
            .collect();

        Enumeration {
            id,
            enumeration_type,
            description,
            values,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum ObjectPropertyType {
    Array(String),
    Boolean,
    Byte,
    Int16,
    Uint16,
    Int32,
    Uint32,
    Int64,
    Uint64,
    Float,
    Double,
    DateTime,
    String,
    Object(String),
    Dictionary { contents: String, key_type: String },
}

impl ObjectPropertyType {
    fn from_str_except_array_or_object(s: &str) -> Option<Self> {
        match s {
            "string" => Some(Self::String),
            "date-time" => Some(Self::DateTime),
            "float" => Some(Self::Float),
            "double" => Some(Self::Double),
            "int16" => Some(Self::Int16),
            "uint16" => Some(Self::Uint16),
            "int32" => Some(Self::Int32),
            "uint32" => Some(Self::Uint32),
            "int64" => Some(Self::Int64),
            "uint64" => Some(Self::Uint64),
            "boolean" => Some(Self::Boolean),
            "byte" => Some(Self::Byte),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct ObjectProperty {
    title: String,
    description: Option<String>,
    property_type: ObjectPropertyType,
    nullable: bool,
    mapped_to: Option<String>,
}

#[derive(Debug)]
struct Object {
    id: String,
    component_dependency: Option<Component>,
    properties: Vec<ObjectProperty>,
}

fn id_to_struct_name(id: &str) -> String {
    // TODO: Turn into proper sturct name
    id.split('.').last().unwrap().to_string()
}

struct MobileManifestEntity {
    id: String,
}

enum Entity {
    Enumeration(Enumeration),
    Object(Object),
    MobileManifestEntity(MobileManifestEntity),
}

impl Entity {
    fn id(&self) -> &String {
        match self {
            Entity::Enumeration(e) => &e.id,
            Entity::Object(o) => &o.id,
            Entity::MobileManifestEntity(m) => &m.id,
        }
    }
    fn bungie_docs_link(&self) -> String {
        let schema = format!("schema_{}", self.id().replace('.', "-"));
        format!(
            "https://bungie-net.github.io/multi/{}.html#{}",
            schema, schema
        )
    }
}

fn get_id(s: &str) -> String {
    lazy_static! {
        static ref RE1: Regex = Regex::new(r#"<a id="[^"]+?"><h3>.+?</h3></a>"#).unwrap();
        static ref RE2: Regex = Regex::new(r#"<h3>.+?</h3>"#).unwrap();
    }
    let a = &RE1.captures(s).unwrap()[0];
    let h3 = RE2.captures(a).unwrap()[0].to_string();
    h3[4..(h3.len() - 5)].to_string()
}

fn get_attributes(s: &str) -> Vec<Attribute> {
    lazy_static! {
        static ref RE1: Regex = Regex::new(
            r#"<div class="attributes">[\r\s.]+?(<span class=".+?">.+?</span>[\r\s]+?)*?</div>"#
        )
        .unwrap();
        static ref RE2: Regex = Regex::new(r#"<span class=".+?">.+?</span>"#).unwrap();
        static ref RE3: Regex = Regex::new(r#"Depends on Component ".+?""#).unwrap();
    }
    let div = &RE1
        .captures(s)
        .unwrap_or_else(|| panic!("Failed RE1 call: {}", s))[0];
    match RE2.captures(div) {
        Some(spans) => {
            let span_count = spans.len();
            let mut attributes = Vec::with_capacity(span_count);
            for i in 0..span_count {
                let span = &spans[i];
                if span.contains("enum") {
                    attributes.push(Attribute::Enumeration)
                } else if span.contains("nullable") {
                    attributes.push(Attribute::Nullable)
                } else if span.contains("manifest-entity") {
                    attributes.push(Attribute::MobileManifestEntity)
                } else if span.contains("depends-on") {
                    let component_string = &RE3.captures(span).unwrap()[0];
                    let component_string = &component_string[22..(component_string.len() - 1)];
                    attributes.push(Attribute::DependsOn(Component::new(component_string)))
                } else if span.contains("mapped") {
                } else {
                    panic!("Unkown attribute: {}", span);
                }
            }
            attributes
        }
        None => Vec::new(),
    }
}

fn get_properties(s: &str) -> Vec<ObjectProperty> {
    lazy_static! {
        static ref RE1: Regex =
            Regex::new(r#"<div class="title"><strong>[a-zA-Z0-9]+?</strong></div>"#).unwrap();
        static ref RE2: Regex =
            Regex::new(r#"<div class="description">[\r\s]+?.+?\.?[\r\s]+?</div>"#).unwrap();
        static ref RE3: Regex = Regex::new(
            r#"<div class="type">[\r\s]+?<strong>Type</strong>: [a-z\-]+?(16|32|64)?[\r\s]+?</div>"#
        )
        .unwrap();
        static ref RE4: Regex = Regex::new(r#"</strong>: [a-z\-]+(16|32|64)?"#).unwrap();
        static ref RE5: Regex = Regex::new(
            r#"<strong>Array Contents:</strong>[\r\s]+?((<a href="[^"]+?">[A-Za-z\.0-9]+?</a>)|([a-z\-]+(16|32|64)?))"#
        )
        .unwrap();
        static ref RE6: Regex = Regex::new(r#">[A-Za-z\.0-9]+?</a>"#).unwrap();
        static ref RE7: Regex = Regex::new(r#"See: <a href="[^"]+?">[A-Za-z\.0-9]+?</a>"#).unwrap();
        static ref RE8: Regex = Regex::new(r#"<strong>Dictionary Contents:</strong>[\r\s]+?((<a href="[^"]+?">[A-Za-z\.0-9]+?</a>)|([a-z\-]+(16|32|64)?))"#).unwrap();
        static ref RE9: Regex = Regex::new(r#"<strong>Dictionary Key Type: </strong>[\r\s]+?[a-z\-]+(16|32|64)?"#).unwrap();
        static ref RE10: Regex = Regex::new(r#"<strong>Mapped to Manifest Database Definition: </strong>[\r\s]+?<a href="[^"]+?">[A-Za-z\.0-9]+?</a>"#).unwrap();
    }
    let mut box_contents_split = s.split("<div class=\"box-contents\">");
    box_contents_split.next(); // Remove the first element, which is the stuff before the first div.
    box_contents_split
        .map(|s| {
            let title = {
                let title = &RE1
                    .captures(s)
                    .unwrap_or_else(|| panic!("Failed RE1 capture: {}", s))[0];
                title[27..(title.len() - 15)].to_string()
            };
            let description = RE2.captures(s).map(|c| {
                let description = &c[0];
                description[25..(description.len() - 6)].trim().to_string()
            });
            let property_type = {
                match RE3.captures(s) {
                    Some(cap) => {
                        let property_type_str = &cap[0];
                        let property_type_str = &RE4.captures(property_type_str).unwrap()[0];
                        let property_type_str = &property_type_str[11..];
                        match property_type_str {
                            "array" => {
                                let array_contents = &RE5
                                    .captures(s)
                                    .unwrap_or_else(|| panic!("Failed RE5 capture: {}", s))[0];
                                match RE6.captures(array_contents) {
                                    Some(cap) => {
                                        let array_contents = &cap[0];
                                        let array_contents = array_contents
                                            [1..(array_contents.len() - 4)]
                                            .to_string();
                                        ObjectPropertyType::Array(array_contents)
                                    }
                                    None => {
                                        let array_contents =
                                            array_contents[32..].trim().to_string();
                                        ObjectPropertyType::Array(array_contents)
                                    }
                                }
                            }
                            _ => match ObjectPropertyType::from_str_except_array_or_object(
                                property_type_str,
                            ) {
                                Some(s) => s,
                                None => match RE8.captures(s) {
                                    Some(cap) => {
                                        let dictionary_contents = &cap[0];
                                        let mut dictionary_contents = match RE6
                                            .captures(dictionary_contents)
                                        {
                                            Some(cap) => {
                                                let dictionary_contents = cap[0].to_string();
                                                dictionary_contents
                                                    [1..(dictionary_contents.len() - 4)]
                                                    .to_string()
                                            }
                                            None => dictionary_contents[32..].trim().to_string(),
                                        };
                                        if dictionary_contents.starts_with("rong>") {
                                            dictionary_contents =
                                                dictionary_contents[6..].trim().to_string();
                                        }
                                        let dictionary_key_type =
                                            RE9.captures(s).unwrap()[0][39..].trim().to_string();
                                        //println!("dictionary_contents: {}", dictionary_contents);
                                        ObjectPropertyType::Dictionary {
                                            contents: dictionary_contents,
                                            key_type: dictionary_key_type,
                                        }
                                    }
                                    None => {
                                        let see = &RE7
                                            .captures(s)
                                            .unwrap_or_else(|| panic!("Failed RE7 capture: {}", s))
                                            [0];
                                        let see = &RE6.captures(see).unwrap()[0];
                                        ObjectPropertyType::Object(
                                            see[1..(see.len() - 4)].to_string(),
                                        )
                                    }
                                },
                            },
                        }
                    }
                    None => {
                        let see = &RE7
                            .captures(s)
                            .unwrap_or_else(|| panic!("Failed RE3/RE7 capture: {}", s))[0];
                        let see = &RE6.captures(see).unwrap()[0];
                        ObjectPropertyType::Object(see[1..(see.len() - 4)].to_string())
                    }
                }
            };
            let nullable = match property_type {
                ObjectPropertyType::Object(_) => false,
                _ => {
                    let attributes = get_attributes(s);
                    if attributes.is_empty() {
                        false
                    } else if attributes.len() == 1 {
                        if attributes[0] == Attribute::Nullable {
                            true
                        } else {
                            panic!("Got unexpected attribute: {:?}", attributes[0])
                        }
                    } else {
                        panic!("Got multiple attributes: {:?}", attributes)
                    }
                }
            };
            let mapped_to = {
                if s.contains("<span class=\"mapped\">") {
                    let mapped_to = &RE10.captures(s).unwrap()[0];
                    let mapped_to = &RE6.captures(mapped_to).unwrap()[0];
                    let mapped_to = &mapped_to[1..(mapped_to.len() - 4)];
                    Some(mapped_to.to_string())
                } else {
                    None
                }
            };
            ObjectProperty {
                title,
                description,
                property_type,
                nullable,
                mapped_to,
            }
        })
        .collect()
}

fn main() {
    let mut entities: Vec<String> = {
        let body = reqwest::blocking::get("https://bungie-net.github.io/")
            .unwrap()
            .text()
            .unwrap();
        let entites_index = body.find("<h2>Entities</h2>").unwrap();
        body[entites_index..].to_string()
    }
    .split("<div class=\"schema box\">")
    .map(|s| s.to_string())
    .collect();
    entities.remove(0);

    let entities: Vec<Entity> = entities
        .into_iter()
        .map(|s| {
            let id = get_id(&s);
            println!("Parsing {}", id);
            let attributes = get_attributes(&s);
            let (is_object, component_dependency) = {
                if attributes.is_empty() {
                    (true, None)
                } else {
                    match attributes[0] {
                        Attribute::Enumeration | Attribute::MobileManifestEntity => (false, None),
                        Attribute::DependsOn(c) => (true, Some(c)),
                        Attribute::Nullable | Attribute::Mapped(_) => panic!(),
                    }
                }
            };
            if is_object {
                Entity::Object(Object {
                    id,
                    component_dependency,
                    properties: get_properties(&s),
                })
            } else {
                match attributes[0] {
                    Attribute::Enumeration => Entity::Enumeration(Enumeration::new(&s)),
                    Attribute::MobileManifestEntity => {
                        Entity::MobileManifestEntity(MobileManifestEntity { id })
                    }
                    Attribute::DependsOn(_) | Attribute::Nullable | Attribute::Mapped(_) => {
                        panic!()
                    }
                }
            }
        })
        .collect();
    let mut objects_generated = 0;
    let mut object_double_brackets_skipped = 0;
    let mut enumerations_generated = 0;
    let mut manifests_generated = 0;
    use std::io::prelude::*;
    let mut output_file = std::fs::File::create("output.rs").unwrap();
    output_file.write_all(b"use std::collections::HashMap;\n\ntype Byte = u8;\npub type Int16 = i16;\npub type Int32 = i32;\npub type Int64 = i64;\npub type Uint16 = u16;\npub type Uint32 = u32;\npub type Uint64 = u64;\npub type Float = f32;\npub type Double: f64;\n\n").unwrap();
    for entity in entities {
        let bungie_docs_link = entity.bungie_docs_link();
        match entity {
            Entity::Enumeration(enumeration) => {
                enumerations_generated += 1;
                output_file.write_all(b"#[repr(").unwrap();
                match enumeration.enumeration_type {
                    EnumerationType::Byte => output_file.write_all(b"Byte").unwrap(),
                    EnumerationType::Int32 => output_file.write_all(b"Int32").unwrap(),
                    EnumerationType::Int64 => output_file.write_all(b"Int64").unwrap(),
                }
                output_file.write_all(b")]\nenum ").unwrap();
                output_file
                    .write_all(id_to_struct_name(&enumeration.id).as_bytes())
                    .unwrap();
                output_file.write_all(b" {").unwrap();
                for value in enumeration.values {
                    output_file.write_all(b"\n\t").unwrap();
                    output_file.write_all(value.name.as_bytes()).unwrap();
                    output_file.write_all(b" = ").unwrap();
                    output_file.write_all(value.value.as_bytes()).unwrap();
                    output_file.write_all(b",").unwrap();
                }
                output_file.write_all(b"\n}\n\n").unwrap();
            }
            Entity::Object(object) => {
                if object.id.ends_with("[]") {
                    //println!("{:?}", object);
                    object_double_brackets_skipped += 1;
                } else {
                    objects_generated += 1;
                    output_file
                        .write_all(b"/// [Bungie documentation](")
                        .unwrap();
                    output_file.write_all(bungie_docs_link.as_bytes()).unwrap();
                    output_file.write_all(b")\n#[derive(Debug, Deserialize)]\n#[serde(rename_all = \"camelCase\")]\npub struct ").unwrap();
                    output_file
                        .write_all(id_to_struct_name(&object.id).as_bytes())
                        .unwrap();
                    output_file.write_all(b" {\n").unwrap();
                    for p in object.properties {
                        let mut is_hash = false;
                        if let Some(mapped_to) = p.mapped_to {
                            is_hash = true;
                            // TODO: Added proper mapped to docs
                            output_file.write_all(b"\t/// mapped to ").unwrap();
                            output_file.write_all(mapped_to.as_bytes()).unwrap();
                            output_file.write_all(b"\n").unwrap();
                        } else if p.property_type == ObjectPropertyType::DateTime {
                            output_file
                                .write_all(b"\t#[serde(deserialize_with = \"from_timestamp\")]\n")
                                .unwrap();
                        }
                        output_file.write_all(b"\t pub ").unwrap();
                        output_file
                            .write_all(
                                {
                                    let mut paramter_title = camel_case_to_snake_case(&p.title);
                                    if paramter_title == "type" {
                                        // TODO: Implement a better version of this
                                        paramter_title.push('_');
                                    }
                                    paramter_title
                                }
                                .as_bytes(),
                            )
                            .unwrap();
                        output_file.write_all(b": ").unwrap();
                        if p.nullable {
                            output_file.write_all(b"Option<").unwrap();
                        }
                        if is_hash {
                            output_file.write_all(b"Hash<").unwrap();
                        }
                        match p.property_type {
                            ObjectPropertyType::Array(id) => {
                                output_file.write_all(b"Vec<").unwrap();
                                output_file
                                    .write_all(id_to_struct_name(&id).as_bytes())
                                    .unwrap();
                                output_file.write_all(b">").unwrap();
                            }
                            ObjectPropertyType::Boolean => output_file.write_all(b"bool").unwrap(),
                            ObjectPropertyType::Byte => output_file.write_all(b"Byte").unwrap(),
                            ObjectPropertyType::Int16 => output_file.write_all(b"Int16").unwrap(),
                            ObjectPropertyType::Uint16 => output_file.write_all(b"Uint16").unwrap(),
                            ObjectPropertyType::Int32 => output_file.write_all(b"Int16").unwrap(),
                            ObjectPropertyType::Uint32 => output_file.write_all(b"Uint32").unwrap(),
                            ObjectPropertyType::Int64 => output_file.write_all(b"Int64").unwrap(),
                            ObjectPropertyType::Uint64 => output_file.write_all(b"Uint64").unwrap(),
                            ObjectPropertyType::Float => output_file.write_all(b"Float").unwrap(),
                            ObjectPropertyType::Double => output_file.write_all(b"Double").unwrap(),
                            ObjectPropertyType::DateTime => {
                                output_file.write_all(b"APIdateTime").unwrap();
                            }
                            ObjectPropertyType::String => output_file.write_all(b"String").unwrap(),
                            ObjectPropertyType::Object(id) => {
                                output_file
                                    .write_all(id_to_struct_name(&id).as_bytes())
                                    .unwrap();
                            }
                            ObjectPropertyType::Dictionary { contents, key_type } => {
                                output_file.write_all(b"HashMap<").unwrap();
                                output_file.write_all(key_type.as_bytes()).unwrap();
                                output_file.write_all(b", ").unwrap();
                                output_file
                                    .write_all(id_to_struct_name(&contents).as_bytes())
                                    .unwrap();
                                output_file.write_all(b">").unwrap();
                            }
                        }
                        if is_hash {
                            output_file.write_all(b">").unwrap();
                        }
                        if p.nullable {
                            output_file.write_all(b">").unwrap();
                        }
                        output_file.write_all(b",\n").unwrap();
                    }
                    output_file.write_all(b"}\n\n").unwrap();
                }
            }
            Entity::MobileManifestEntity(manifest) => {
                manifests_generated += 1;
            }
        }
    }
    println!(
        "\n\nObjects generated: {}\nEnumerations generated: {}\nManifests not generated: {}\nObject[] things skipped: {}",
        objects_generated, enumerations_generated, manifests_generated, object_double_brackets_skipped
    );
}
