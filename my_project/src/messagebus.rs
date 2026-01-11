use crate::{configures, events};

pub async fn headle(event: events::Event) -> Result<String, String> {
    let db = configures::get_config().database.get_connection().await;

    let mut queue = vec![event];

    while queue.len() > 0 {
        let mut tx = db.begin().await.map_err(|e| e.to_string())?;
        let ev = queue.remove(0);
        match ev {
            events::Event::BatchCreate(e) => match crate::handlers::add_batch(e, &mut tx).await {
                Ok(_) => {
                    tx.commit().await.map_err(|e| e.to_string())?;
                    return Ok("Batch created successfully".to_string());
                }
                Err(err) => {
                    tx.rollback().await.map_err(|e| e.to_string())?;
                    return Err(err.to_string());
                }
            },
            events::Event::AllocateRequired(e) => match crate::handlers::allocate(e, &mut tx).await
            {
                Ok(Some((message, version))) => {
                    tx.commit().await.map_err(|e| e.to_string())?;
                    return Ok(message);
                }
                Ok(None) => {
                    tx.rollback().await.map_err(|e| e.to_string())?;
                    return Err("Allocation failed: None returned".to_string());
                }
                Err(err) => {
                    tx.rollback().await.map_err(|e| e.to_string())?;
                    return Err(err.to_string());
                }
            },
            events::Event::OutOfStock(e) => {
                match crate::handlers::send_out_of_stock_notification(e, &mut tx).await {
                    Ok(_) => {
                        tx.commit().await.map_err(|e| e.to_string())?;
                        return Ok("Out of stock notification sent".to_string());
                    }
                    Err(err) => {
                        tx.rollback().await.map_err(|e| e.to_string())?;
                        return Err(err.to_string());
                    }
                }
            }
        }
    }

    Ok("Event handled successfully".to_string())
}
