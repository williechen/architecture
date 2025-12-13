use chrono::*;
use serde_json::Value;
use serde_json::map::Map;
use sqlx::AnyPool;
use sqlx::Column;
use sqlx::Row;
use sqlx::ValueRef;
use sqlx::any::AnyValue;
use sqlx::decode::Decode; // 必需

pub fn decode_any_value(raw: AnyValue) -> Value {
    println!("Decoding value: {:?}", raw);

    Value::Null
}

/// 主函式：執行任意 SQL，將每一列轉成 serde_json::Value（動態欄位）
pub async fn query_to_json_try_decode(
    pool: &AnyPool,
    sql: &str,
) -> Result<Vec<Value>, sqlx::Error> {
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

        out.push(Value::Object(obj));
    }

    Ok(out)
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
        let results = query_to_json_try_decode(&pool, "SELECT * FROM test_table")
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
