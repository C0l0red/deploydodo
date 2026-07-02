#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
pub struct ContainerInfo {
    pub id: String,
    pub name: String,
    pub image: String,
}

#[derive(Debug, Clone)]
pub enum TerminalOutput {
    Stdout(String),
    Stderr(String),
}
