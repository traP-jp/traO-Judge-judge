use std::collections::HashMap;

use anyhow::Ok;
use axum::async_trait;
use judge_core::model::{dep_name_repository::DepNameRepository, identifiers::DepId};
use sqlx::{MySqlPool, QueryBuilder};

use crate::model::dep_name::DepNameRow;



#[derive(Clone)]
pub struct DepNameRepositoryImpl {
    pool: MySqlPool,
}


impl DepNameRepositoryImpl {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}



#[async_trait]
impl DepNameRepository<i64> for DepNameRepositoryImpl {
    async fn insert_many(
        &self,
        problem_id: i64,
        dep_id_to_name: HashMap<DepId, String>,
    ) -> anyhow::Result<()> {
        let mut query_builder = QueryBuilder::new(
            "INSERT INTO dep_name (problem_id, dep_id, name) VALUES ",
        );

        let mut separated = query_builder.separated(", ");
        for (dep_id, name) in dep_id_to_name {
            separated.push("(");
            separated.push_bind_unseparated(problem_id);
            separated.push_unseparated(", ");
            separated.push_bind_unseparated(dep_id.into());
            separated.push_unseparated(", ");
            separated.push_bind_unseparated(name);
            separated.push_unseparated(")");
        }

        query_builder.build().execute(&self.pool).await?;
        Ok(())
    }

    async fn get_many(
        &self,
        dep_ids: Vec<DepId>,
    ) -> anyhow::Result<HashMap<DepId, Option<String>>> {
        let mut query_builder = QueryBuilder::new(
            "SELECT dep_id, name FROM dep_name WHERE dep_id IN (",
        );

        let mut separated = query_builder.separated(", ");
        for dep_id in &dep_ids {
            separated.push_bind(dep_id.into());
        }
        query_builder.push(")");

        let dep_names = query_builder
            .build_query_as::<DepNameRow>()
            .fetch_all(&self.pool)
            .await?;

        let mut dep_id_to_name = HashMap::new();
        for dep_name in dep_names {
            dep_id_to_name.insert(dep_name.dep_id.into(), Some(dep_name.name));
        }

        for dep_id in dep_ids {
            if !dep_id_to_name.contains_key(&dep_id) {
                dep_id_to_name.insert(dep_id, None);
            }
        }

        Ok(dep_id_to_name)
    }

    async fn remove_many(&self, problem_id: i64) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM dep_name WHERE problem_id = ?")
            .bind(problem_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn get_many_by_problem_id(
        &self,
        problem_id: i64,
    ) -> anyhow::Result<HashMap<DepId, String>> {
        let dep_names : Vec<DepNameRow> = sqlx::query_as::<_, DepNameRow>(
            "SELECT dep_id, name FROM dep_name WHERE problem_id = ?")
        .bind(problem_id)
        .fetch_all(&self.pool)
        .await?;

        let mut dep_id_to_name = HashMap::new();
        for dep_name in dep_names {
            dep_id_to_name.insert(dep_name.dep_id.into(), dep_name.name);
        }

        Ok(dep_id_to_name)
    }
}