use crate::prelude::{GKey, GValue, Map};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Bulk {
    pub map: HashMap<GKey, GValue>,
    pub occurrences: usize,
}

impl Into<HashMap<GKey, GValue>> for crate::prelude::Bulk {
    fn into(self) -> HashMap<GKey, GValue> {
        let mut map = HashMap::new();
        map.insert(
            GKey::String("data".to_string()),
            GValue::Map(Map::from(self.map)),
        );
        map.insert(
            GKey::String("occurrences".to_string()),
            GValue::Int64(self.occurrences as i64),
        );
        map
    }
}

impl Into<Map> for crate::prelude::Bulk {
    fn into(self) -> Map {
        let map: HashMap<GKey, GValue> = self.into();
        Map::from(map)
    }
}
