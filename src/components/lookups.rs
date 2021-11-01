use std::collections::HashMap;

use super::druid_types::DruidNativeType;

#[derive(Debug, Clone)]
pub enum Lookup {
    Map {
        map: HashMap<DruidNativeType, DruidNativeType>,
        injective: Option<bool>,
    },
}
