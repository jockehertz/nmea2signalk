use serde_json;

pub struct Update: {
    pub path: String,
    pub value: serde_json::Value
}
pub struct Delta: {
    pub context: String,
    pub update: Vec<Update>
}
