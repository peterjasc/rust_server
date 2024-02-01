use crate::model::user::User;
use sqlx::PgPool;

pub async fn find_user_by_id(id: i32, pool: &PgPool) -> Result<Option<User>, sqlx::Error> {
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
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|row| User {
        id: row.id,
        username: row.username,
    }))
}

pub async fn insert_user(user: &User, pool: &PgPool) -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;

    sqlx::query!(
        r#"
            INSERT INTO users (id, username)
            VALUES ($1, $2)
            "#,
        user.id,
        user.username,
    )
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await
}
