use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,
    Paid,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
    Refunded
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderItem {
    pub product_id: String,
    pub product_name: String,
    pub quantity: u32,
    pub unit_price: u32,
    pub total_price: u32
}

impl OrderItem {
    pub fn new(product_id: Uuid, product_name: String, quantity: u32, unit_price: u32) -> Self {
        Self {
            product_id: product_id.to_string(),
            product_name,
            quantity,
            unit_price,
            total_price: unit_price * quantity
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub items: Vec<OrderItem>,
    pub status: OrderStatus,
    pub total_amount: u32,
    pub shipping_address: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Order {
    fn calculate_total_amount(items: &Vec<OrderItem>) -> u32 {
        items.iter().map(|item| item.total_price).sum()
    }

    pub fn new(user_id: Uuid, items: Vec<OrderItem>, shipping_address: String) -> Self {
        let total_amount = Order::calculate_total_amount(&items);

        Self {
            id: Uuid::new_v4(),
            user_id,
            items,
            status: OrderStatus::Pending,
            total_amount,
            shipping_address,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    pub fn get_total_amount(&self) -> u32 {
       Order::calculate_total_amount(&self.items)
    }

    pub fn total_quantity(&self) -> u32 {
        self.items.iter().map(|item| item.quantity).sum()
    }

    pub fn can_be_cancelled(&self) -> bool {
        matches!(self.status, OrderStatus::Pending | OrderStatus::Paid)
    }

    pub fn can_be_refunded(&self) -> bool {
        matches!(self.status, OrderStatus::Paid | OrderStatus::Shipped)
    }

    pub fn mark_as_paid(&mut self) -> Result<(), String> {
        if !matches!(self.status, OrderStatus::Pending) {
            return Err("Only pending orders can be marked as paid".to_string())
        }

        self.status = OrderStatus::Paid;
        self.updated_at = chrono::Utc::now();
        Ok(())
    }

    pub fn cancel(&mut self) -> Result<(), String> {
        if !self.can_be_cancelled() {
            return Err(format!("Order cannot be cancelled with status {:?}", self.status))
        }

        self.status = OrderStatus::Cancelled;
        self.updated_at = chrono::Utc::now();
        Ok(())
    }

    pub fn update_status(&mut self, status: OrderStatus) {
        self.status = status;
        self.updated_at = chrono::Utc::now();
    }
}