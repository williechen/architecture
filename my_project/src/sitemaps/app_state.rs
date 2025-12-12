use crate::entities::ssm_codemap::SsmCodemap;
use crate::entities::ssm_config::SsmConfig;
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_cron_scheduler::Job;
use tokio_cron_scheduler::JobScheduler;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub codemap: Arc<RwLock<HashMap<String, HashMap<String, String>>>>,
    pub config: Arc<RwLock<HashMap<String, HashMap<String, String>>>>,
}

pub async fn load(state: AppState) {
    let scheduler = JobScheduler::new().await.unwrap();

    let db = state.db.clone();
    let codemap = state.codemap.clone();
    let config = state.config.clone();

    let job = Job::new_async("0 */15 * * * *", move |_id, _lock| {
        let db_cache = db.clone();
        let codemap_cache = codemap.clone();
        let config_cache = config.clone();

        Box::pin({
            async move {
                tracing::debug!("定期更新快取資料...");

                // 模擬更新資料，例如呼叫 API 或 DB
                let codemap = load_codemap(&db_cache).await;
                let config = load_config(&db_cache).await;

                let mut codemap_data = codemap_cache.write().await;
                *codemap_data = codemap;
                let mut config_data = config_cache.write().await;
                *config_data = config;

                tracing::info!("快取更新完成");
            }
        })
    })
    .unwrap();

    scheduler.add(job).await.unwrap();
    scheduler.start().await.unwrap();
}

pub async fn load_codemap(db: &SqlitePool) -> HashMap<String, HashMap<String, String>> {
    tracing::debug!("selecting codemap...");

    let mut codemap = HashMap::new();
    let items: Vec<SsmCodemap> = SsmCodemap::select_all(db).await.unwrap();
    for item in items {
        let group = item.category;
        let key = item.code;
        let value = item.name;
        codemap
            .entry(group)
            .or_insert_with(HashMap::new)
            .insert(key, value);
    }
    tracing::info!("codemap loaded: {:?}", codemap);
    codemap
}

pub async fn load_config(db: &SqlitePool) -> HashMap<String, HashMap<String, String>> {
    tracing::info!("selecting config...");

    let mut config = HashMap::new();
    let items: Vec<SsmConfig> = SsmConfig::select_all(db).await.unwrap();
    for item in items {
        let group = item.category;
        let key = item.code;
        let value = item.name;
        config
            .entry(group)
            .or_insert_with(HashMap::new)
            .insert(key, value);
    }
    tracing::info!("config loaded: {:?}", config);
    config
}
