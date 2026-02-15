#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlnKey {
    Section,
    Role,
    Rights,
    Capabilities,
    Species,
    ReversalPolicy,
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct AlnProperty {
    pub key: AlnKey,
    pub value: String,
}

#[derive(Debug, Clone, Default)]
pub struct AlnDocument {
    pub properties: Vec<AlnProperty>,
}

impl AlnDocument {
    pub fn get_values(&self, key: &AlnKey) -> Vec<&str> {
        self.properties
            .iter()
            .filter(|p| &p.key == key)
            .map(|p| p.value.as_str())
            .collect()
    }
}
