#![forbid(unsafe_code)]

mod db;
mod id;
mod quote;

pub use crate::db::*;
pub use crate::id::*;
pub use crate::quote::*;

/// Database tests.
#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::ops::Deref;

    /// A test database. When dropped, it will be deleted.
    struct TestDB {
        /// The name of the da
        /// The inner database.
        inner: DB,
    }

    impl TestDB {
        /// Create a new test database.
        pub async fn new() -> Result<Self> {
            let db_id = new_id();
            let db_name = format!("test_{}", db_id);
            let db = DB::create(&db_name).await?;

            Ok(Self { inner: db })
        }

        /// Delete the database.
        pub async fn delete(self) -> Result<()> {
            self.inner.delete().await
        }
    }

    impl Deref for TestDB {
        type Target = DB;

        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }

    /// Test the database quote functions.
    #[tokio::test]
    async fn test_quote() {
        // Init
        let db = TestDB::new().await.unwrap();
        sqlx::query(
            "
            CREATE TABLE IF NOT EXISTS quotes (
                id TEXT NOT NULL,
                quote TEXT NOT NULL
            );
            ",
        )
        .execute(&**db)
        .await
        .unwrap();

        let quotes = [
            "We're not free in what we do because we're not free in what we want.",
            "Give a man a gun and he can rob a bank, but give a man a bank, and he can rob the world.",
            "Where I once would fear the cost of truth, now I only ask: What is the cost of lies?",
        ];

        let mut db_quotes = Vec::new();

        // Create/get
        for quote in &quotes {
            let q1 = Quote::create(&db, quote).await;
            assert_eq!(q1.quote, quote.to_owned());
            let q2 = Quote::get(&db, &q1.id).await.unwrap();
            assert_eq!(q1, q2);
            db_quotes.push(q1);
        }

        // List
        let quote_list = Quote::list(&db).await;
        assert_eq!(quote_list.len(), db_quotes.len());

        for quote in &db_quotes {
            assert!(quote_list.contains(quote));
        }

        // Set
        let first_quote = db_quotes.first_mut().unwrap();
        first_quote.set(&db, "The brain is wider than the sky\nFor put them side by side\nThe one the other will contain\nWith ease, and you beside").await;
        let get_first_quote = Quote::get(&db, &first_quote.id).await.unwrap();
        assert_eq!(get_first_quote, *first_quote);
        let list_first_quote = quote_list.first().unwrap();
        assert_eq!(get_first_quote.id, list_first_quote.id);
        assert_ne!(get_first_quote, *list_first_quote);

        // Delete
        get_first_quote.delete(&db).await;
        let get_original_first_quote = Quote::get(&db, &first_quote.id).await;
        assert!(get_original_first_quote.is_none());
        let new_quote_list = Quote::list(&db).await;
        assert_eq!(new_quote_list.len(), quote_list.len() - 1);

        for quote in &quote_list {
            if quote.id != first_quote.id {
                assert!(new_quote_list.contains(quote));
            } else {
                assert!(!new_quote_list.contains(quote));
            }
        }

        // Clean up
        db.delete().await.unwrap();
    }
}
