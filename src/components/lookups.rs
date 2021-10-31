use std::collections::HashMap;

use super::druid_types::DruidType;

#[derive(Debug, Clone)]
pub enum Lookup {
    Map {
        map: HashMap<DruidType, DruidType>,
        injective: Option<bool>,
    },
}
