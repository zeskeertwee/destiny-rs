use crate::models::types::*;

pub trait ComponentID {
    fn component_id(&self) -> Int32;
}