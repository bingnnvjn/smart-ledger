//! 记录管理模块

use super::{Record, RecordType};
use crate::error::{AppError, AppResult};
use crate::storage::database::Database;

/// 添加记录
pub fn add_record(
    db: &Database,
    amount: f64,
    record_type: RecordType,
    category_id: i64,
    account_id: i64,
    note: &str,
) -> AppResult<Record> {
    if amount <= 0.0 {
        return Err(AppError::InvalidParam("金额必须大于0".to_string()));
    }

    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    let id = db.execute(
        "INSERT INTO records (amount, record_type, category_id, account_id, note, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        rusqlite::params![amount, record_type.to_string(), category_id, account_id, note, now, now],
    )?;

    // 更新账户余额
    match record_type {
        RecordType::Expense => {
            db.execute(
                "UPDATE accounts SET balance = balance - ?1 WHERE id = ?2",
                rusqlite::params![amount, account_id],
            )?;
        }
        RecordType::Income => {
            db.execute(
                "UPDATE accounts SET balance = balance + ?1 WHERE id = ?2",
                rusqlite::params![amount, account_id],
            )?;
        }
        RecordType::Transfer => {
            // 转账逻辑需要额外处理
        }
    }

    Ok(Record {
        id,
        amount,
        record_type,
        category_id,
        account_id,
        note: note.to_string(),
        created_at: now.clone(),
        updated_at: now,
    })
}

/// 获取记录列表
pub fn get_records(
    db: &Database,
    start_date: &str,
    end_date: &str,
    record_type: Option<RecordType>,
    limit: Option<i64>,
) -> AppResult<Vec<Record>> {
    let mut sql = String::from(
        "SELECT id, amount, record_type, category_id, account_id, note, created_at, updated_at 
         FROM records WHERE created_at BETWEEN ?1 AND ?2"
    );
    
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = vec![
        Box::new(start_date.to_string()),
        Box::new(end_date.to_string()),
    ];

    if let Some(rt) = record_type {
        sql.push_str(" AND record_type = ?3");
        params.push(Box::new(rt.to_string()));
    }

    sql.push_str(" ORDER BY created_at DESC");

    if let Some(l) = limit {
        sql.push_str(&format!(" LIMIT {}", l));
    }

    let records = db.query(&sql, params)?;
    Ok(records)
}

/// 删除记录
pub fn delete_record(db: &Database, record_id: i64) -> AppResult<()> {
    // 先获取记录信息
    let record = get_record_by_id(db, record_id)?;
    
    // 恢复账户余额
    match record.record_type {
        RecordType::Expense => {
            db.execute(
                "UPDATE accounts SET balance = balance + ?1 WHERE id = ?2",
                rusqlite::params![record.amount, record.account_id],
            )?;
        }
        RecordType::Income => {
            db.execute(
                "UPDATE accounts SET balance = balance - ?1 WHERE id = ?2",
                rusqlite::params![record.amount, record.account_id],
            )?;
        }
        _ => {}
    }

    db.execute("DELETE FROM records WHERE id = ?1", rusqlite::params![record_id])?;
    Ok(())
}

/// 根据ID获取记录
pub fn get_record_by_id(db: &Database, record_id: i64) -> AppResult<Record> {
    let sql = "SELECT id, amount, record_type, category_id, account_id, note, created_at, updated_at FROM records WHERE id = ?1";
    let record = db.query_one(sql, rusqlite::params![record_id])?;
    record.ok_or_else(|| AppError::NotFound(format!("记录 {} 不存在", record_id)))
}

impl ToString for RecordType {
    fn to_string(&self) -> String {
        match self {
            RecordType::Expense => "expense".to_string(),
            RecordType::Income => "income".to_string(),
            RecordType::Transfer => "transfer".to_string(),
        }
    }
}

impl std::str::FromStr for RecordType {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "expense" => Ok(RecordType::Expense),
            "income" => Ok(RecordType::Income),
            "transfer" => Ok(RecordType::Transfer),
            _ => Err(AppError::InvalidParam(format!("无效的记录类型: {}", s))),
        }
    }
}
