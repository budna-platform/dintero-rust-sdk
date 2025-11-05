use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Metadata = HashMap<String, serde_json::Value>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct MetadataMap(pub Metadata);

impl MetadataMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: impl Serialize,
    ) -> Option<serde_json::Value> {
        let value = serde_json::to_value(value).ok()?;
        self.0.insert(key.into(), value)
    }

    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.0.get(key)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<HashMap<String, serde_json::Value>> for MetadataMap {
    fn from(map: HashMap<String, serde_json::Value>) -> Self {
        Self(map)
    }
}

impl From<MetadataMap> for HashMap<String, serde_json::Value> {
    fn from(metadata: MetadataMap) -> Self {
        metadata.0
    }
}
