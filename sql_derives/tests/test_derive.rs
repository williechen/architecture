use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
use sql_derives::SqlTable;

#[test]
fn test_sql_table_macro() {
    #[derive(SqlTable)]
    #[sql(table = "user")]
    struct UacUser {
        id: i32,
        #[sql(column = "user_name")]
        name: String,
        email: String,
        created_at: NaiveDateTime,
    }
    let user = UacUser {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
        created_at: DateTime::<Utc>::from_timestamp(1625077800, 0)
            .unwrap()
            .naive_utc(),
    };
    assert_eq!(UacUser::table_name(), "user");
    assert_eq!(
        UacUser::columns(),
        vec!["id", "user_name", "email", "created_at"]
    );
    assert_eq!(
        UacUser::select_sql(None),
        "SELECT id, user_name, email, created_at FROM user WHERE 1=1 "
    );
    assert_eq!(
        user.insert_sql(),
        "INSERT INTO user (id, user_name, email, created_at) VALUES (1, 'Alice', 'alice@example.com', '2021-06-30 18:30:00')"
    );
    assert_eq!(
        user.update_sql(Some("id=1")),
        "UPDATE user SET id=1, user_name='Alice', email='alice@example.com', created_at='2021-06-30 18:30:00' WHERE id=1"
    );

    assert_eq!(UacUser::where_eq("id", "1"), "id=1");
    assert_eq!(
        UacUser::where_and(vec!["id=1".to_string(), "user_name='Alice'".to_string()]),
        "(id=1 AND user_name='Alice')"
    );
    assert_eq!(
        UacUser::where_or(vec!["id=1".to_string(), "user_name='Alice'".to_string()]),
        "(id=1 OR user_name='Alice')"
    );
    assert_eq!(
        UacUser::where_and(vec![
            UacUser::where_eq("id", "1"),
            UacUser::where_or(vec![
                UacUser::where_eq("user_name", "'Alice'"),
                UacUser::where_eq("email", "'alice@example.com'"),
            ]),
        ]),
        "(id=1 AND (user_name='Alice' OR email='alice@example.com'))"
    );
}
