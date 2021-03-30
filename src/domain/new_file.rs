use sqlx::types::Uuid;

pub struct NewFile {
    pub id: Uuid,
    pub name: String,
    pub url: String,
}
