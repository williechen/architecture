use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use serde_json::Map;
use serde_json::Value as JsonValue;
use sqlx::AnyPool;
use sqlx::Column;
use sqlx::Row;
use sqlx::Value; // ★ 必須
use sqlx::ValueRef;
use sqlx::any::AnyQueryResult;
use sqlx::any::AnyValue;

fn decode_any_value(raw: AnyValue) -> JsonValue {
    // 整數
    if let Ok(opt) = raw.try_decode::<Option<i64>>() {
        return opt.map(JsonValue::from).unwrap_or(JsonValue::Null);
    }
    if let Ok(opt) = raw.try_decode::<Option<i32>>() {
        return opt.map(JsonValue::from).unwrap_or(JsonValue::Null);
    }
    if let Ok(opt) = raw.try_decode::<Option<i16>>() {
        return opt.map(JsonValue::from).unwrap_or(JsonValue::Null);
    }

    // 浮點
    if let Ok(opt) = raw.try_decode::<Option<f64>>() {
        return opt.map(JsonValue::from).unwrap_or(JsonValue::Null);
    }
    if let Ok(opt) = raw.try_decode::<Option<f32>>() {
        return opt
            .map(|v| JsonValue::from(v as f64))
            .unwrap_or(JsonValue::Null);
    }

    // bool
    if let Ok(opt) = raw.try_decode::<Option<bool>>() {
        return opt.map(JsonValue::from).unwrap_or(JsonValue::Null);
    }

    // 字串
    if let Ok(opt) = raw.try_decode::<Option<String>>() {
        return opt.map(JsonValue::from).unwrap_or(JsonValue::Null);
    }

    // bytes → base64（新版 API）
    if let Ok(opt) = raw.try_decode::<Option<Vec<u8>>>() {
        return match opt {
            None => JsonValue::Null,
            Some(bytes) => JsonValue::String(STANDARD.encode(bytes)),
        };
    }

    // fallback: 無法 decode，回傳類型名稱
    JsonValue::String(format!("<unsupported: {}>", raw.type_info()))
}

/// 主函式：執行任意 SQL，將每一列轉成 serde_json::Value（動態欄位）
pub async fn read_to_json(pool: &AnyPool, sql: &str) -> Result<Vec<JsonValue>, sqlx::Error> {
    let rows = sqlx::query(sql).fetch_all(pool).await?;

    let mut out = Vec::with_capacity(rows.len());

    for row in rows {
        let mut obj = Map::new();

        // 透過 index 取得 raw value（避免 name 重複 / alias 問題）
        for (i, col) in row.columns().iter().enumerate() {
            let name = (*col).name().to_string();
            let raw = row.try_get_raw(i)?;
            let val = decode_any_value(ValueRef::to_owned(&raw));
            obj.insert(name, val.into());
        }

        out.push(JsonValue::Object(obj));
    }

    Ok(out)
}

pub async fn read_one_to_json(pool: &AnyPool, sql: &str) -> Result<Option<JsonValue>, sqlx::Error> {
    let row_opt = sqlx::query(sql).fetch_optional(pool).await?;

    if let Some(row) = row_opt {
        let mut obj = Map::new();

        for (i, col) in row.columns().iter().enumerate() {
            let name = (*col).name().to_string();
            let raw = row.try_get_raw(i)?;
            let val = decode_any_value(ValueRef::to_owned(&raw));
            obj.insert(name, val.into());
        }

        Ok(Some(JsonValue::Object(obj)))
    } else {
        Ok(None)
    }
}

pub async fn read<T>(pool: &AnyPool, sql: &str) -> Result<Vec<T>, sqlx::Error>
where
    T: for<'r> sqlx::FromRow<'r, sqlx::any::AnyRow> + Send + Unpin,
{
    let rows: Vec<T> = sqlx::query_as(sql).fetch_all(pool).await?;
    Ok(rows)
}

pub async fn read_one<T>(pool: &AnyPool, sql: &str) -> Result<Option<T>, sqlx::Error>
where
    T: for<'r> sqlx::FromRow<'r, sqlx::any::AnyRow> + Send + Unpin,
{
    let row_opt: Option<T> = sqlx::query_as(sql).fetch_optional(pool).await?;
    Ok(row_opt)
}

pub async fn create(pool: &AnyPool, sql: &str) -> Result<AnyQueryResult, sqlx::Error> {
    let query_result = sqlx::query(sql).execute(pool).await?;
    Ok(query_result)
}

pub async fn update(pool: &AnyPool, sql: &str) -> Result<AnyQueryResult, sqlx::Error> {
    let query_result = sqlx::query(sql).execute(pool).await?;
    Ok(query_result)
}

pub async fn delete(pool: &AnyPool, sql: &str) -> Result<AnyQueryResult, sqlx::Error> {
    let query_result = sqlx::query(sql).execute(pool).await?;
    Ok(query_result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use sqlx::Executor;
    use sqlx::any::AnyPoolOptions;
    use sqlx::any::install_default_drivers;

    #[tokio::test]
    async fn test_query_to_json_try_decode() {
        install_default_drivers();
        let pool = AnyPoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        pool.execute(
            "CREATE TABLE test_table (
                id INTEGER PRIMARY KEY,
                name TEXT,
                age INTEGER,
                active INTEGER
            )",
        )
        .await
        .unwrap();
        pool.execute(
            "INSERT INTO test_table (name, age, active) VALUES
            ('Alice', 30, 1),
            ('Bob', NULL, 0)",
        )
        .await
        .unwrap();
        let results = read_to_json(&pool, "SELECT * FROM test_table")
            .await
            .unwrap();
        let expected = vec![
            json!({
                "id": 1,
                "name": "Alice",
                "age": 30,
                "active": 1,
            }),
            json!({
                "id": 2,
                "name": "Bob",
                "age": null,
                "active": 0,
            }),
        ];
        assert_eq!(results, expected);
    }
}
