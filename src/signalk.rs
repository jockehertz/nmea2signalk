use serde_json::Value;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Update {
    pub path: String,
    pub value: Value
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delta {
    pub context: String,
    #[serde( skip_serializing_if = "Vec::is_empty" )]
    pub updates: Vec<Update>
}

impl Delta {

    pub fn new(context: impl Into<String>) -> Self {
        Delta {
            context: context.into(),
            updates: Vec::new(),
        }
    }

    pub fn add_update(mut self, path: impl Into<String>, value: Value) -> Self {
        self.updates.push(Update {
            path: path.into(),
            value
        });
        self
    }

    pub fn add_number(self, path: impl Into<String>, value: f64) -> Self {
        self.add_update(path, serde_json::json!(value))
    }

    pub fn add_string(self, path: impl Into<String>, value: impl Into<String>) -> Self {
        self.add_update(path, serde_json::json!(value.into()))
    }

    pub fn add_bool(self, path: impl Into<String>, value: bool) -> Self {
        self.add_update(path, Value::Bool(value))
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}
