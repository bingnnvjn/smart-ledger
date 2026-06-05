use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RecordType {
    Expense,
    Income,
    Transfer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    pub id: i64,
    pub amount: f64,
    pub record_type: String,
    pub category_id: i64,
    pub account_id: i64,
    pub note: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub icon: String,
    pub category_type: String,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: i64,
    pub name: String,
    pub icon: String,
    pub balance: f64,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyStats {
    pub year: i32,
    pub month: u32,
    pub total_income: f64,
    pub total_expense: f64,
    pub balance: f64,
}
