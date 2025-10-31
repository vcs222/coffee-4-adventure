use crate::error::ApiResult;
use std::path::Path;
use surrealdb::{Surreal, engine::any::Any};
use tokio::fs;

pub type Db = Surreal<Any>;

#[cfg(not(feature = "test-db"))]
pub async fn connect() -> ApiResult<Db> {
    use std::env;
    use surrealdb::opt::auth::Root;

    dotenvy::dotenv().ok();

    let surreal_url = env::var("SURREAL_URL").unwrap_or("memory".into());
    let surreal_ns = env::var("SURREAL_NS")?;
    let surreal_db = env::var("SURREAL_DB")?;
    let surreal_username = env::var("SURREAL_USERNAME")?;
    let surreal_password = env::var("SURREAL_PASSWORD")?;

    let db = surrealdb::engine::any::connect(&surreal_url).await?;
    db.use_ns(&surreal_ns).use_db(&surreal_db).await?;
    db.signin(Root {
        username: &surreal_username,
        password: &surreal_password,
    })
    .await?;

    Ok(db)
}

#[cfg(feature = "test-db")]
pub async fn connect() -> ApiResult<Db> {
    let db = surrealdb::engine::any::connect("mem://").await?;
    db.use_ns("test").use_db("test").await?;
    Ok(db)
}

pub async fn apply_migrations(db: &Db) -> ApiResult<()> {
    let migrations_path = Path::new("./coffee_shared/migrations");

    let mut entries = fs::read_dir(migrations_path).await?;
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.is_file() && path.extension().is_some_and(|ext| ext == "surql") {
            let query = fs::read_to_string(&path).await?;
            db.query(query).await?;
            println!("Applied migration: {}", path.display());
        }
    }
    Ok(())
}
