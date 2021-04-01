use std::fs::read_to_string;
use crate::models::*;
use serde_json;

#[test]
fn general_user_deserialization() {
    let data = read_to_string("./test_data/GeneralUser.json").unwrap();
    let _: user::GeneralUser = serde_json::from_str(&data).unwrap();
}