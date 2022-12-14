pub mod checker;

#[derive(Debug)]
pub struct EnvVarDeclaration {
    pub name: String,
    pub optional: bool,
    pub default: Option<String>,
    pub description: Option<String>,
}

pub struct EnvVar {
    pub name: String,
    pub value: Option<String>,
}
