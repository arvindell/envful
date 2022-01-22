pub mod checker;
pub mod runner;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Envful {
    pub variables: Vec<String>,
}

pub struct EnvVar {
    pub name: String,
    pub value: String,
    pub required: bool,
    pub default: Option<String>,
    pub description: Option<String>,
}
