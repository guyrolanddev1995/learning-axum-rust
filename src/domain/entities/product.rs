use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub category_id: Uuid,
    pub description: Option<String>,
    pub price: u32,
    pub stock: u32,
    pub sku: String,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Product {
    pub fn new(
        category_id: Uuid,
        name: String,
        description: Option<String>,
        price: u32,
        stock: u32,
        sku: String,
    ) -> Self {
        let now = chrono::Utc::now();

        Self {
            id: Uuid::new_v4(),
            category_id,
            name,
            description,
            price,
            stock,
            sku,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_available(&self) -> bool {
        self.is_active && self.stock > 0
    }

    pub fn can_fulfill_order(&self, quantity: u32 ) -> bool {
        self.stock >= quantity
    }

    pub fn reduce_stock(&mut self, quantity: u32) -> Result<(), String> {
        if !self.can_fulfill_order(quantity) {
            return Err(format!(
                "Insufficient stock. Available: {}, Requested: {}",
                self.stock, quantity
            ));
        }

        self.stock -= quantity;
        self.updated_at = chrono::Utc::now();
        Ok(())
    }

    pub fn increase_stock(&mut self, quantity: u32) {
        self.stock += quantity;
        self.updated_at = chrono::Utc::now();
    }

    pub fn calculate_total_price(&self) -> u32 {
        self.price * self.stock
    }

    pub fn update_category(&mut self, category_id: Uuid) {
        self.category_id = category_id;
        self.updated_at = chrono::Utc::now();
    }
    pub fn update_sku(&mut self, sku: String) {
        self.sku = sku;
        self.updated_at = chrono::Utc::now();
    }

    pub fn update_name(&mut self, name: String) {
        self.name = name;
        self.updated_at = chrono::Utc::now();
    }

    pub fn update_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn update_price(&mut self, price: u32) {
        self.price = price;
        self.updated_at = chrono::Utc::now();
    }

    pub fn update_stock(&mut self, stock: u32) {
        self.stock = stock;
    }

    pub fn activate(&mut self) {
        self.is_active = true; self.updated_at = chrono::Utc::now();
    }

    pub fn deactivate(&mut self) {
        self.is_active = false; self.updated_at = chrono::Utc::now();
    }
}