//! 分类管理模块

use super::{Category, RecordType};
use crate::error::{AppError, AppResult};
use crate::storage::database::Database;

/// 预设分类
const DEFAULT_EXPENSE_CATEGORIES: &[(&str, &str)] = &[
    ("餐饮", "🍜"),
    ("交通", "🚗"),
    ("购物", "🛒"),
    ("居住", "🏠"),
    ("娱乐", "🎮"),
    ("通讯", "📱"),
    ("医疗", "💊"),
    ("教育", "📚"),
    ("服饰", "👔"),
    ("美容", "💄"),
    ("影视", "🎬"),
    ("旅行", "✈️"),
];

const DEFAULT_INCOME_CATEGORIES: &[(&str, &str)] = &[
    ("工资", "💰"),
    ("奖金", "🎁"),
    ("投资", "📈"),
    ("兼职", "💼"),
    ("其他", "📋"),
];

/// 初始化默认分类
pub fn init_default_categories(db: &Database) -> AppResult<()> {
    // 检查是否已有分类
    let count: i64 = db.query_scalar("SELECT COUNT(*) FROM categories")?;
    if count > 0 {
        return Ok(());
    }

    // 插入支出分类
    for (i, (name, icon)) in DEFAULT_EXPENSE_CATEGORIES.iter().enumerate() {
        db.execute(
            "INSERT INTO categories (name, icon, category_type, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![name, icon, "expense", i as i32],
        )?;
    }

    // 插入收入分类
    for (i, (name, icon)) in DEFAULT_INCOME_CATEGORIES.iter().enumerate() {
        db.execute(
            "INSERT INTO categories (name, icon, category_type, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![name, icon, "income", i as i32],
        )?;
    }

    Ok(())
}

/// 获取所有分类
pub fn get_categories(db: &Database, category_type: Option<RecordType>) -> AppResult<Vec<Category>> {
    let sql = match category_type {
        Some(_) => "SELECT id, name, icon, category_type, sort_order FROM categories WHERE category_type = ?1 ORDER BY sort_order",
        None => "SELECT id, name, icon, category_type, sort_order FROM categories ORDER BY sort_order",
    };

    let params: Vec<Box<dyn rusqlite::types::ToSql>> = match category_type {
        Some(rt) => vec![Box::new(rt.to_string())],
        None => vec![],
    };

    let categories = db.query(sql, params)?;
    Ok(categories)
}

/// 添加分类
pub fn add_category(
    db: &Database,
    name: &str,
    icon: &str,
    category_type: RecordType,
) -> AppResult<Category> {
    let max_order: i32 = db.query_scalar(
        "SELECT COALESCE(MAX(sort_order), 0) FROM categories WHERE category_type = ?1",
        rusqlite::params![category_type.to_string()],
    )?;

    let id = db.execute(
        "INSERT INTO categories (name, icon, category_type, sort_order) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![name, icon, category_type.to_string(), max_order + 1],
    )?;

    Ok(Category {
        id,
        name: name.to_string(),
        icon: icon.to_string(),
        category_type,
        sort_order: max_order + 1,
    })
}

/// 删除分类
pub fn delete_category(db: &Database, category_id: i64) -> AppResult<()> {
    // 检查是否有关联的记录
    let count: i64 = db.query_scalar(
        "SELECT COUNT(*) FROM records WHERE category_id = ?1",
        rusqlite::params![category_id],
    )?;

    if count > 0 {
        return Err(AppError::InvalidParam("该分类下有记录，无法删除".to_string()));
    }

    db.execute("DELETE FROM categories WHERE id = ?1", rusqlite::params![category_id])?;
    Ok(())
}
