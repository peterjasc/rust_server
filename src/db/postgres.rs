use anyhow::Context;
use sqlx::{Executor, PgPool, Postgres, Transaction};
use crate::model::user::User;

pub async fn find_user_by_id(id: i32, pool: &PgPool) -> Result<User, anyhow::Error> {
    let row = sqlx::query!(
            r#"
            SELECT
                id,
                username
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .context("Failed to find user")?;

    Ok(User { id: row.id, username: row.username })
}

pub async fn insert_user(
    user: &User, mut transaction: Transaction<'_, Postgres>) -> Result<(), anyhow::Error> {
    let query = sqlx::query!(
            r#"
            INSERT INTO users (id, username)
            VALUES ($1, $2)
            "#,
            user.id,
            user.username,
        );

    transaction.execute(query)
        .await
        .context("Failed to insert user")?;

    transaction
        .commit()
        .await
        .context("Failed to commit user")?;

    Ok(())
}