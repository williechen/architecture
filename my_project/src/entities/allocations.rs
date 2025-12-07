
use rbatis::crud;
use rbatis::rbdc::db::ExecResult; // Import the macro

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Allocation {
    pub id: String,
    pub batch_id: String,
    pub order_line_id: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

crud!(Allocation {}, "allocations");

impl Default for Allocation {
    fn default() -> Self {
        Allocation {
            id: "".to_string(),
            batch_id: "".to_string(),
            order_line_id: "".to_string(),
            created_at: DateTime::default(),
            updated_at: DateTime::default(),
        }
    }
}

impl Allocation {
    pub fn new(batch_id: String, order_line_id: String) -> Self {
        Allocation {
            id: xid::new().to_string(),
            batch_id,
            order_line_id,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }

    pub async fn find_all(rb: &rbatis::RBatis) -> rbatis::Result<Vec<Allocation>> {
        Allocation::select_all(rb).await
    }

    pub async fn get(rb: &rbatis::RBatis, id: &str) -> rbatis::Result<Option<Allocation>> {
        let allocations = Allocation::select_by_map(rb, rbs::value::Value::from(id)).await;
        match allocations {
            Ok(list) => {
                if list.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(list[0].clone()))
                }
            }
            Err(e) => Err(e),
        }
    }

    pub async fn create(
        rb: &rbatis::RBatis,
        allocation: &Allocation,
    ) -> rbatis::Result<ExecResult> {
        Allocation::insert(rb, allocation).await
    }

    pub async fn remove(rb: &rbatis::RBatis, id: &str) -> rbatis::Result<ExecResult> {
        Allocation::delete_by_map(rb, rbs::value::Value::from(id)).await
    }
}
