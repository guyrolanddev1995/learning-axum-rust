use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransactionType {
    Payment,
    Refund,
    Charge
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Success,
    Failed,
    Cancelled
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PaymentMethod {
    CreditCard,
    DebitCard,
    Paypal,
    Cash,
    BankTransfer
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub order_id: Uuid,
    pub user_id: Uuid,
    pub transaction_type: TransactionType,
    pub payment_method: PaymentMethod,
    pub amount: u32,
    pub status: TransactionStatus,
    pub payment_gateway_id: Option<String>,
    pub error_message: Option<String>,
    pub create_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Transaction {
    pub fn new(
        order_id: Uuid,
        user_id: Uuid,
        amount: u32,
        payment_method: PaymentMethod
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            order_id,
            user_id,
            transaction_type: TransactionType::Payment,
            payment_method,
            amount,
            status: TransactionStatus::Pending,
            payment_gateway_id: None,
            error_message: None,
            create_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    pub fn is_successful(&self) -> bool {
        matches!(self.status, TransactionStatus::Success)
    }

    pub fn is_pending(&self) -> bool {
        matches!(self.status, TransactionStatus::Pending)
    }

    pub fn can_be_refunded(&self) -> bool {
        self.is_successful() && matches!(self.transaction_type, TransactionType::Payment)
    }

    pub fn mark_as_success(&mut self, gateway_id: String) {
        self.status = TransactionStatus::Success;
        self.payment_gateway_id = Some(gateway_id);
        self.error_message = None;
        self.updated_at = chrono::Utc::now();
    }

    pub fn mark_as_failed(&mut self, error_message: String) {
        self.status = TransactionStatus::Failed;
        self.error_message = Some(error_message);
        self.updated_at = chrono::Utc::now();
    }

    pub fn create_refund(&self, refund_amount: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            order_id: self.order_id,
            user_id: self.user_id,
            transaction_type: TransactionType::Refund,
            payment_method: self.payment_method.clone(),
            amount: refund_amount,
            status: TransactionStatus::Pending,
            payment_gateway_id: None,
            error_message: None,
            create_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
}