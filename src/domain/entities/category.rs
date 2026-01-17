use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub slug: String,
    pub parent_id: Option<Uuid>,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Category {
    pub fn new(name: String, description: String, parent_id: Option<Uuid>) -> Self {
        let now = chrono::Utc::now();
        let slug = Self::generate_slug(&name);

        Self {
            id: Uuid::new_v4(),
            name,
            description,
            slug,
            parent_id,
            is_active: true,
            created_at: now,
            updated_at: Some(now),
        }
    }

    pub fn is_root_category(&self) -> bool{
        self.parent_id.is_none()
    }

    pub fn has_parent(&self) -> bool {
        self.parent_id.is_some()
    }

    pub fn can_be_parent(&self) -> bool {
        self.is_active
    }

    pub fn activate(&mut self) { self.is_active = true; }

    pub fn deactivate(&mut self) { self.is_active = false; }

    pub fn generate_slug(name: &str) -> String {
        name.to_lowercase()
            .replace(' ', "-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect()
    }
}