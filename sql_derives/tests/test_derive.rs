#[test]
fn test_to_snake_case() {
    assert_eq!(to_snake_case("MyStruct"), "my_struct");
    assert_eq!(to_snake_case("AnotherExample"), "another_example");
    assert_eq!(to_snake_case("Test"), "test");
}

#[test]
fn test_sql_table_macro() {
    #[derive(SqlTable)]
    struct User {
        id: i32,
        #[sql(column = "user_name")]
        name: String,
        email: String,
    }
    let user = User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };
    assert_eq!(User::table_name(), "user");
    assert_eq!(
        User::columns(),
        vec![
            "id",
            "user_name",
            "email
"
        ]
    );
    assert_eq!(User::select_sql(), "SELECT id, user_name, email FROM user");
    assert_eq!(
        user.insert_sql(),
        "INSERT INTO user (id, user_name, email) VALUES ('1', 'Alice', 'alice@example.com')"
    );
    assert_eq!(
        user.update_sql(Some("id='1'")),
        "UPDATE user SET id='1', user_name='Alice', email='alice@example.com' WHERE id='1'"
    );
}
