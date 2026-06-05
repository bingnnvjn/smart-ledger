//! 预算管理模块

use super::Budget;
use crate::error::{AppError, AppResult};
use crate::storage::database::Database;

/// 设置预算
pub fn set_budget(
    db: &Database,
    category_id: i64,
    amount: f64,
    month: &str,
) -> AppResult<Budget> {
    if amount <= 0.0 {
        return Err(AppError::InvalidParam("预算金额必须大于0".to_string()));
    }

    // 检查是否已存在
    let existing: Option<i64> = db.query_optional(
        "SELECT id FROM budgets WHERE category_id = ?1 AND month = ?2",
        rusqlite::params![category_id, month],
    )?;

    if let Some(id) = existing {
        // 更新
        db.execute(
            "UPDATE budgets SET amount = ?1 WHERE id = ?2",
            rusqlite::params![amount, id],
        )?;
    } else {
        // 新增
        db.execute(
            "INSERT INTO budgets (category_id, amount, month) VALUES (?1, ?2, ?3)",
            rusqlite::params![category_id, amount, month],
        )?;
    }

    // 获取本月已花费
    let spent = get_budget_spent(db, category_id, month)?;

    Ok(Budget {
        id: existing.unwrap_or(0),
        category_id,
        amount,
        month: month.to_string(),
        spent,
    })
}

/// 获取预算列表
pub fn get_budgets(db: &Database, month: &str) -> AppResult<Vec<Budget>> {
    let sql = "SELECT id, category_id, amount, month FROM budgets WHERE month = ?1";
    let budgets: Vec<Budget> = db.query(sql, rusqlite::params![month])?;

    let mut result = Vec::new();
    for mut budget in budgets {
        budget.spent = get_budget_spent(db, budget.category_id, month)?;
        result.push(budget);
    }

    Ok(result)
}

/// 获取预算已花费
fn get_budget_spent(db: &Database, category_id: i64, month: &str) -> AppResult<f64> {
    let start_date = format!("{}-01 00:00:00", month);
    let end_date = format!("{}-31 23:59:59", month);

    let spent: f64 = db.query_scalar(
        "SELECT COALESCE(SUM(amount), 0) FROM records WHERE category_id = ?1 AND record_type = 'expense' AND created_at BETWEEN ?2 AND ?3",
        rusqlite::params![category_id, start_date, end_date],
    )?;

    Ok(spent)
}

/// 删除预算
pub fn delete_budget(db: &Database, budget_id: i64) -> AppResult<()> {
    db.execute("DELETE FROM budgets WHERE id = ?1", rusqlite::params![budget_id])?;
    Ok(())
}

/// 获取预算预警
pub fn get_budget_alerts(db: &Database, month: &str, threshold: f64) -> AppResult<Vec<Budget>> {
    let budgets = get_budgets(db, month)?;
    
    let alerts: Vec<Budget> = budgets
        .into_iter()
        .filter(|b| b.spent / b.amount >= threshold)
        .collect();

    Ok(alerts)
}
