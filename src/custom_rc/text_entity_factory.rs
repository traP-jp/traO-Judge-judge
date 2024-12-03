mod dynamic_lru;
use crate::custom_rc::file_entity::TextFileEntity;
use crate::text_resource_repository::TextResourceRepository as RepoTrait;
use anyhow::{Context, Result};
use byte_unit::Byte;
use dynamic_lru::DynamicallySizedLRUCache;
use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;

pub struct TextEntityFactory<
    ExternalAccessKey: Eq + std::hash::Hash + Clone + ToString,
    RepoType: RepoTrait<ExternalAccessKey>,
> {
    cache: DynamicallySizedLRUCache<ExternalAccessKey, Arc<TextFileEntity>>,
    cache_directory: PathBuf,
    cache_directory_size_maximum: u64,
    repo: RepoType,
    _phantom: std::marker::PhantomData<ExternalAccessKey>,
}

impl<
        ExternalAccessKey: Eq + std::hash::Hash + Clone + ToString,
        RepoType: RepoTrait<ExternalAccessKey>,
    > TextEntityFactory<ExternalAccessKey, RepoType>
{
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
            cache: DynamicallySizedLRUCache::new(),
            cache_directory,
            cache_directory_size_maximum: (cache_directory_size_limit.as_u64() as f64
                * cache_dir_margin_ratio) as u64,
            repo,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn get_text_file_entity(
        &mut self,
        text_resource_id: ExternalAccessKey,
    ) -> Result<Arc<TextFileEntity>> {
        if let Some(entity) = self.cache.get(&text_resource_id) {
            return Ok(entity.clone());
        } else {
            let text = self.repo.get_text(&text_resource_id).with_context(|| {
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
            let entity = Arc::new(TextFileEntity::new(path, &text)?);
            self.cache.insert(text_resource_id, entity.clone());
            while self.pop_if_needed()? {}
            Ok(entity)
        }
    }

    fn pop_if_needed(&mut self) -> Result<bool> {
        let dir_size_u64 = std::fs::metadata::<PathBuf>(self.cache_directory.clone())
            .with_context(|| {
                format!(
                    "Failed to get metadata of cache directory : {:?}",
                    self.cache_directory
                )
            })?
            .len();
        if dir_size_u64 > self.cache_directory_size_maximum {
            self.cache.remove_one()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
