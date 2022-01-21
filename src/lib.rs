pub mod checker;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Envful {
    pub variables: Vec<String>,
}

pub struct EnvVar(String, String);
