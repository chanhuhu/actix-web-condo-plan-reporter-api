#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct NewIssue {
    pub name: String,
    pub description: String,
    pub location: String,
}
