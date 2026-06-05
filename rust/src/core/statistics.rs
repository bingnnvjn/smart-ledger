//! 统计模块

use super::{MonthlyStats, CategoryStat, RecordType};
use crate::error::AppResult;
use crate::storage::database::Database;

/// 获取月度统计
pub fn get_monthly_stats(db: &Database, year: i32, month: u32) -> AppResult<MonthlyStats> {
    let month_str = format!("{:04}-{:02}", year, month);
    let start_date = format!("{}-01 00:00:00", month_str);
    let end_date = format!("{}-31 23:59:59", month_str);

    // 获取总收入
    let total_income: f64 = db.query_scalar(
        "SELECT COALESCE(SUM(amount), 0) FROM records WHERE record_type = 'income' AND created_at BETWEEN ?1 AND ?2",
        rusqlite::params![start_date, end_date],
    )?;

    // 获取总支出
    let total_expense: f64 = db.query_scalar(
        "SELECT COALESCE(SUM(amount), 0) FROM records WHERE record_type = 'expense' AND created_at BETWEEN ?1 AND ?2",
        rusqlite::params![start_date, end_date],
    )?;

    // 获取分类统计
    let category_stats = get_category_stats(db, year, month)?;

    Ok(MonthlyStats {
        year,
        month,
        total_income,
        total_expense,
        balance: total_income - total_expense,
        category_stats,
    })
}

/// 获取分类统计
pub fn get_category_stats(db: &Database, year: i32, month: u32) -> AppResult<Vec<CategoryStat>> {
    let month_str = format!("{:04}-{:02}", year, month);
    let start_date = format!("{}-01 00:00:00", month_str);
    let end_date = format!("{}-31 23:59:59", month_str);

    let sql = "
        SELECT 
            c.id as category_id,
            c.name as category_name,
            COALESCE(SUM(r.amount), 0) as amount
        FROM categories c
        LEFT JOIN records r ON r.category_id = c.id 
            AND r.record_type = 'expense'
            AND r.created_at BETWEEN ?1 AND ?2
        WHERE c.category_type = 'expense'
        GROUP BY c.id
        HAVING amount > 0
        ORDER BY amount DESC
    ";

    let rows: Vec<(i64, String, f64)> = db.query_raw(sql, rusqlite::params![start_date, end_date])?;
    
    let total: f64 = rows.iter().map(|(_, _, amount)| amount).sum();

    let category_stats = rows
        .into_iter()
        .map(|(category_id, category_name, amount)| CategoryStat {
            category_id,
            category_name,
            amount,
            percentage: if total > 0.0 { amount / total * 100.0 } else { 0.0 },
        })
        .collect();

    Ok(category_stats)
}

/// 获取最近N个月的趋势
pub fn get_monthly_trend(db: &Database, months: u32) -> AppResult<Vec<(String, f64, f64)>> {
    let mut result = Vec::new();
    let now = chrono::Local::now();
    
    for i in 0..months {
        let date = now - chrono::Duration::days(i as i64 * 30);
        let year = date.format("%Y").to_string();
        let month = date.format("%m").to_string();
        
        let stats = get_monthly_stats(db, year.parse()?, month.parse()?)?;
        result.push((format!("{}-{}", year, month), stats.total_income, stats.total_expense));
    }

    result.reverse();
    Ok(result)
}
