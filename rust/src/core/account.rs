//! 账户管理模块

use super::Account;
use crate::error::{AppError, AppResult};
use crate::storage::database::Database;

/// 预设账户
const DEFAULT_ACCOUNTS: &[(&str, &str)] = &[
    ("现金", "💵"),
    ("银行卡", "💳"),
    ("支付宝", "📱"),
    ("微信", "💬"),
];

/// 初始化默认账户
pub fn init_default_accounts(db: &Database) -> AppResult<()> {
    let count: i64 = db.query_scalar("SELECT COUNT(*) FROM accounts")?;
    if count > 0 {
        return Ok(());
    }

    for (i, (name, icon)) in DEFAULT_ACCOUNTS.iter().enumerate() {
        db.execute(
            "INSERT INTO accounts (name, icon, balance, sort_order) VALUES (?1, ?2, 0, ?3)",
            rusqlite::params![name, icon, i as i32],
        )?;
    }

    Ok(())
}

/// 获取所有账户
pub fn get_accounts(db: &Database) -> AppResult<Vec<Account>> {
    let sql = "SELECT id, name, icon, balance, sort_order FROM accounts ORDER BY sort_order";
    let accounts = db.query(sql, vec![])?;
    Ok(accounts)
}

/// 添加账户
pub fn add_account(db: &Database, name: &str, icon: &str) -> AppResult<Account> {
    let max_order: i32 = db.query_scalar("SELECT COALESCE(MAX(sort_order), 0) FROM accounts")?;

    let id = db.execute(
        "INSERT INTO accounts (name, icon, balance, sort_order) VALUES (?1, ?2, 0, ?3)",
        rusqlite::params![name, icon, max_order + 1],
    )?;

    Ok(Account {
        id,
        name: name.to_string(),
        icon: icon.to_string(),
        balance: 0.0,
        sort_order: max_order + 1,
    })
}

/// 删除账户
pub fn delete_account(db: &Database, account_id: i64) -> AppResult<()> {
    let count: i64 = db.query_scalar(
        "SELECT COUNT(*) FROM records WHERE account_id = ?1",
        rusqlite::params![account_id],
    )?;

    if count > 0 {
        return Err(AppError::InvalidParam("该账户有交易记录，无法删除".to_string()));
    }

    db.execute("DELETE FROM accounts WHERE id = ?1", rusqlite::params![account_id])?;
    Ok(())
}

/// 获取总余额
pub fn get_total_balance(db: &Database) -> AppResult<f64> {
    let total: f64 = db.query_scalar("SELECT COALESCE(SUM(balance), 0) FROM accounts")?;
    Ok(total)
}
