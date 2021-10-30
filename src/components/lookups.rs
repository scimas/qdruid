use std::collections::HashMap;

use super::druid_types::DruidType;

pub enum Lookup {
    Map {
        map: HashMap<DruidType, DruidType>,
        injective: Option<bool>,
    },
}
