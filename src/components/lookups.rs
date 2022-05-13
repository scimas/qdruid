use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use super::druid_types::DruidNativeType;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Lookup {
    Map {
        map: HashMap<String, DruidNativeType>,
        injective: Option<bool>,
    },
}

impl Lookup {
    pub fn map(map: HashMap<String, DruidNativeType>, injective: Option<bool>) -> Self {
        Self::Map { map, injective }
    }
}
