//! Postgres integration smoke test (same role as a Laravel RefreshDatabase feature test).
//! Requires DATABASE_URL; CI crates job provides Postgres 16.

#[sqlx::test]
async fn inserts_a_review_row(pool: sqlx::PgPool) -> sqlx::Result<()> {
    let id: i64 = sqlx::query_scalar(
        r#"
        INSERT INTO reviews (product_id, rating, body)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
    )
    .bind(1_i64)
    .bind(5_i32)
    .bind("excellent")
    .fetch_one(&pool)
    .await?;

    assert!(id > 0);

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM reviews WHERE product_id = $1")
        .bind(1_i64)
        .fetch_one(&pool)
        .await?;
    assert_eq!(count, 1);
    Ok(())
}
