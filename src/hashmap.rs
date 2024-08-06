use anyhow::Result;
use dashmap::DashMap;
use log::{info, warn};
use sqlx::{prelude::FromRow, PgPool};

#[derive(Debug, FromRow)]
pub struct DbRow {
    pub key: String,
    pub value: Vec<u8>,
}

impl DbRow {
    pub fn new(key: &str, value: Vec<u8>) -> Self {
        Self {
            key: key.to_owned(),
            value,
        }
    }

    async fn insert(&self, db_connecton_pool: &PgPool) -> Result<()> {
        sqlx::query!(
            "INSERT INTO hashmap (key, value) VALUES ($1, $2)",
            self.key,
            self.value
        )
        .execute(db_connecton_pool)
        .await?;

        Ok(())
    }

    async fn update(&self, db_connecton_pool: &PgPool) -> Result<()> {
        sqlx::query!(
            "UPDATE hashmap SET value = $1 WHERE key = $2",
            self.value,
            self.key
        )
        .execute(db_connecton_pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, db_connecton_pool: &PgPool) -> Result<()> {
        sqlx::query!("DELETE FROM hashmap WHERE key = $1", self.key)
            .execute(db_connecton_pool)
            .await?;

        Ok(())
    }

    async fn get(key: &str, db_connecton_pool: &PgPool) -> Result<Option<Self>> {
        let row = sqlx::query_as!(DbRow, "SELECT key, value FROM hashmap WHERE key = $1", key)
            .fetch_optional(db_connecton_pool)
            .await?;

        Ok(row)
    }
}

#[derive(Debug)]
pub struct HashMap {
    map: DashMap<String, Vec<u8>>,
    db_connecton_pool: PgPool,
}

impl HashMap {
    pub fn new(db_connecton_pool: PgPool) -> Self {
        Self {
            map: DashMap::new(),
            db_connecton_pool,
        }
    }

    pub async fn get(&self, key: &str) -> Option<Vec<u8>> {
        info!("GET, with key {}", key);
        if let Some(value) = self.map.get(key) {
            info!("Cache hit");
            return Some(value.clone());
        }

        info!("Cache miss, getting from database");
        if let Some(row) = DbRow::get(&key.to_string(), &self.db_connecton_pool)
            .await
            .ok()?
        {
            let value = row.value;
            self.map.insert(key.to_owned(), value.clone());
            return Some(value);
        }

        warn!("Key not found in database");
        None
    }

    pub async fn set(&self, key: &str, value: Vec<u8>) -> Result<()> {
        info!("POST, with key {}", key);
        info!("Updating database");
        let row = DbRow::new(key, value.clone());
        info!("Updating cache");
        row.insert(&self.db_connecton_pool).await?;
        self.map.insert(key.to_owned(), value);

        Ok(())
    }

    pub async fn update(&self, key: &str, value: Vec<u8>) -> Result<()> {
        info!("PUT, with key {}", key);
        info!("Updating database");
        let row = DbRow::new(key, value.clone());
        info!("Updating cache");
        row.update(&self.db_connecton_pool).await?;
        self.map.insert(key.to_owned(), value);

        Ok(())
    }

    pub async fn delete(&self, key: &str) -> Result<()> {
        info!("DELETE, with key {}", key);
        info!("Deleting from database");
        let row = DbRow::new(key, vec![]);
        info!("Deleting from cache");
        row.delete(&self.db_connecton_pool).await?;
        self.map.remove(key);

        Ok(())
    }
}
