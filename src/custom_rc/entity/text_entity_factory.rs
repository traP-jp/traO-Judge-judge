mod dynamic_lru;
use super::file_entity::TextFileEntity;
use crate::text_resource_repository::TextResourceRepository as RepoTrait;
use anyhow::{Context, Result};
use byte_unit::Byte;
use dynamic_lru::DynamicallySizedLRUCache;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

pub struct TextEntityFactory<RepoType: RepoTrait> {
    cache: Mutex<DynamicallySizedLRUCache<Uuid, Arc<TextFileEntity>>>,
    cache_directory: PathBuf,
    cache_directory_size_maximum: u64,
    repo: RepoType,
    _phantom: std::marker::PhantomData<Uuid>,
}

impl<RepoType: RepoTrait> TextEntityFactory<RepoType> {
    pub fn new(
        cache_directory: PathBuf,
        cache_directory_size_limit: Byte,
        repo: RepoType,
        cache_dir_margin_ratio: f64,
    ) -> Self {
        if 0.0 >= cache_dir_margin_ratio || cache_dir_margin_ratio >= 1.0 {
            panic!("cache_dir_margin_ratio must be in the range (0.0, 1.0)");
        }
        Self {
            cache: Mutex::new(DynamicallySizedLRUCache::new()),
            cache_directory,
            cache_directory_size_maximum: (cache_directory_size_limit.as_u64() as f64
                * cache_dir_margin_ratio) as u64,
            repo,
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn get_text_file_entity(
        &self,
        text_resource_id: Uuid,
        cache: bool,
    ) -> Result<Arc<TextFileEntity>> {
        if let Some(entity) = self.cache.lock().await.get(&text_resource_id) {
            return Ok(entity.clone());
        }
        let text = self
            .repo
            .get_text(&text_resource_id)
            .await
            .with_context(|| {
                format!(
                    "Failed to get text from repository : {:?}",
                    text_resource_id.to_string()
                )
            })?;
        let path = self.cache_directory.join(format!(
            "{:?}-{:?}",
            text_resource_id.to_string().chars().filter(|c| {
                match c {
                    'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => true,
                    _ => false,
                }
            },),
            Uuid::new_v4().to_string()
        ));
        let entity = Arc::new(TextFileEntity::new(path, &text).await?);
        if cache {
            self.cache
                .lock()
                .await
                .insert(text_resource_id, entity.clone());
        }
        while self
            .pop_if_needed()
            .await
            .context("Failed to pop resource from cache")?
        {}
        Ok(entity)
    }

    async fn pop_if_needed(&self) -> Result<bool> {
        let dir_size_u64 = std::fs::metadata::<PathBuf>(self.cache_directory.clone())
            .with_context(|| {
                format!(
                    "Failed to get metadata of cache directory : {:?}",
                    self.cache_directory
                )
            })?
            .len();
        if dir_size_u64 > self.cache_directory_size_maximum {
            self.cache.lock().await.remove_one()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
