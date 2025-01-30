use super::{users::UserId, Repository};

impl Repository {
    pub async fn save_user_password(&self, id: UserId, password: &str) -> anyhow::Result<()> {
        let hash = bcrypt::hash(password, self.bcrypt_cost)?;

        sqlx::query("INSERT INTO users_passwords (user_id, password) VALUES (?, ?)")
            .bind(id)
            .bind(&hash)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn update_user_password(&self, id: UserId, password: &str) -> anyhow::Result<()> {
        let hash = bcrypt::hash(password, self.bcrypt_cost)?;

        sqlx::query("UPDATE users_passwords SET password = ? WHERE user_id = ?")
            .bind(&hash)
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn verify_user_password(&self, id: UserId, password: &str) -> anyhow::Result<bool> {
        let hash = sqlx::query_scalar::<_, String>(
            "SELECT password FROM users_passwords WHERE user_id = ?",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(bcrypt::verify(password, &hash)?)
    }
}
