#![forbid(unsafe_code)]

mod account;
mod account_transaction;
mod account_transaction_tag;
mod account_type;
mod budget;
mod category;
mod db;
mod id;
mod reminder;
mod report_template;
mod save;
mod subcategory;
mod tag;
mod timeframe;

pub use crate::account::*;
pub use crate::account_transaction::*;
pub use crate::account_transaction_tag::*;
pub use crate::account_type::*;
pub use crate::budget::*;
pub use crate::category::*;
pub use crate::db::DB;
pub use crate::reminder::*;
pub use crate::report_template::*;
pub use crate::save::{Save, SaveMetadata};
pub use crate::subcategory::*;
pub use crate::tag::*;
pub use crate::timeframe::*;

use crate::id::*;

/// The database tables, in order.
pub(crate) const TABLES: &[&str] = &[
    "account",
    "reminder",
    "budget",
    "category",
    "subcategory",
    "account_transaction",
    "tag",
    "account_transaction_tag",
    "report_template",
];

/// Converts a name into an acceptable file name.
pub(crate) fn convert_file_name(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c
            } else if c == ' ' || c == '_' {
                '_'
            } else {
                '-'
            }
        })
        .collect()
}

#[cfg(test)]
pub(crate) use tests::*;

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::ops::{Deref, DerefMut};

    /// A test database.
    pub struct TestDB {
        /// The inner database.
        inner: DB,
    }

    impl TestDB {
        /// Creates a new test database.
        pub async fn new() -> Result<Self> {
            let db_id = new_id();
            let db_name = format!("test_{db_id}");
            let db = DB::create(&db_name).await?;

            Ok(Self { inner: db })
        }

        /// Deletes the test database.
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

    impl DerefMut for TestDB {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.inner
        }
    }
}
