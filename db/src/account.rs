use crate::{new_id, AccountType, DB};
use chrono::{NaiveDateTime, Utc};

/// A representation of an account in the database.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Account {
    /// The account's identifier.
    pub id: String,
    /// The account type.
    pub account_type: String,
    /// The name of the account.
    pub name: String,
    /// A description of the account.
    pub description: Option<String>,
    /// When the account was created.
    pub created_at: NaiveDateTime,
    /// When the account was last edited.
    pub edited_at: Option<NaiveDateTime>,
    /// When the account was last reconciled.
    pub reconciled_at: Option<NaiveDateTime>,
}

impl Account {
    /// Creates a new account.
    pub async fn create(
        db: &mut DB,
        account_type: AccountType,
        name: &str,
        description: &str,
    ) -> Self {
        let id = new_id();
        let account_type_name = account_type.to_internal_name();

        sqlx::query!(
            "INSERT INTO account (id, account_type, name, description) VALUES (?, ?, ?, ?);",
            id,
            account_type_name,
            name,
            description
        )
        .execute(&mut **db)
        .await
        .unwrap();

        Self::get(db, &id).await.unwrap()
    }

    /// Gets an account from the database.
    pub async fn get(db: &mut DB, id: &str) -> Option<Self> {
        sqlx::query_as!(Self, "SELECT * FROM account WHERE id = ?;", id)
            .fetch_optional(&mut **db)
            .await
            .unwrap()
    }

    /// Lists all accounts in the database.
    pub async fn list(db: &mut DB) -> Vec<Self> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM account ORDER BY edited_at DESC, created_at DESC;"
        )
        .fetch_all(&mut **db)
        .await
        .unwrap()
    }

    /// Gets the account type.
    pub fn get_account_type(&self) -> AccountType {
        AccountType::from_internal_name(&self.account_type).unwrap()
    }

    /// Marks the account as edited.
    pub async fn mark_edited(&mut self, db: &mut DB) {
        self.edited_at = Some(Utc::now().naive_utc());

        sqlx::query!(
            "UPDATE account SET edited_at = ? WHERE id = ?;",
            self.edited_at,
            self.id
        )
        .execute(&mut **db)
        .await
        .unwrap();
    }

    /// Marks the account as reconciled.
    pub async fn mark_reconciled(&mut self, db: &mut DB) {
        self.reconciled_at = Some(Utc::now().naive_utc());

        sqlx::query!(
            "UPDATE account SET reconciled_at = ? WHERE id = ?;",
            self.reconciled_at,
            self.id
        )
        .execute(&mut **db)
        .await
        .unwrap();
    }

    /// Sets the account type.
    pub async fn set_account_type(&mut self, db: &mut DB, account_type: AccountType) {
        self.account_type = account_type.to_internal_name();

        sqlx::query!(
            "UPDATE account SET account_type = ? WHERE id = ?;",
            self.account_type,
            self.id
        )
        .execute(&mut **db)
        .await
        .unwrap();

        self.mark_edited(db).await;
    }

    /// Sets the account name.
    pub async fn set_name(&mut self, db: &mut DB, name: &str) {
        self.name = name.to_owned();

        sqlx::query!(
            "UPDATE account SET name = ? WHERE id = ?;",
            self.name,
            self.id
        )
        .execute(&mut **db)
        .await
        .unwrap();

        self.mark_edited(db).await;
    }

    /// Sets the account description.
    pub async fn set_description(&mut self, db: &mut DB, description: &str) {
        self.description = Some(description.to_owned());

        sqlx::query!(
            "UPDATE account SET description = ? WHERE id = ?;",
            self.description,
            self.id
        )
        .execute(&mut **db)
        .await
        .unwrap();

        self.mark_edited(db).await;
    }

    /// Deletes the account from the database.
    pub async fn delete(self, db: &mut DB) {
        sqlx::query!("DELETE FROM account WHERE id = ?;", self.id)
            .execute(&mut **db)
            .await
            .unwrap();
    }
}

/// Account tests.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::TestDB;

    #[tokio::test]
    async fn test_account() {
        // Init
        let mut db = TestDB::new().await.unwrap();

        // Create
        let mut account1 = Account::create(
            &mut db,
            AccountType::BankAccount,
            "My Bank Account",
            "Description of bank account",
        )
        .await;
        let mut account2 = Account::create(
            &mut db,
            AccountType::CreditCard,
            "My Credit Card",
            "Description of credit card",
        )
        .await;

        // Get
        let account3 = Account::get(&mut db, &account1.id).await.unwrap();
        assert_eq!(account3, account1);
        let account4 = Account::get(&mut db, &account2.id).await.unwrap();
        assert_eq!(account4, account2);
        assert!(Account::get(&mut db, "").await.is_none());

        // List
        let accounts = Account::list(&mut db).await;
        assert_eq!(accounts.len(), 2);
        let account5 = accounts.iter().find(|x| x.id == account1.id).unwrap();
        assert_eq!(account5, &account1);
        let account6 = accounts.iter().find(|x| x.id == account2.id).unwrap();
        assert_eq!(account6, &account2);

        // Get account type
        assert_eq!(account1.get_account_type(), AccountType::BankAccount);
        assert_eq!(account2.get_account_type(), AccountType::CreditCard);

        // Mark edited
        assert!(account1.edited_at.is_none());
        account1.mark_edited(&mut db).await;
        assert!(account1.edited_at.is_some());
        assert_ne!(account1, account3);

        // Mark reconciled
        assert!(account2.reconciled_at.is_none());
        account2.mark_reconciled(&mut db).await;
        assert!(account2.reconciled_at.is_some());
        assert_ne!(account2, account4);

        // Set account type
        account1
            .set_account_type(&mut db, AccountType::Investment)
            .await;
        assert_eq!(account1.get_account_type(), AccountType::Investment);
        let account7 = Account::get(&mut db, &account1.id).await.unwrap();
        assert_eq!(account7, account1);

        // Set account name
        account1.set_name(&mut db, "My Investments").await;
        assert_eq!(&account1.name, "My Investments");
        let account8 = Account::get(&mut db, &account1.id).await.unwrap();
        assert_eq!(account8, account1);

        // Set account description
        account1
            .set_description(&mut db, "Investment description")
            .await;
        assert_eq!(
            account1.description.as_ref().unwrap(),
            "Investment description"
        );
        let account9 = Account::get(&mut db, &account1.id).await.unwrap();
        assert_eq!(account9, account1);

        // Delete
        let account_id1 = account1.id.clone();
        assert!(Account::get(&mut db, &account_id1).await.is_some());
        account1.delete(&mut db).await;
        assert!(Account::get(&mut db, &account_id1).await.is_none());
        let account_id2 = account2.id.clone();
        assert!(Account::get(&mut db, &account_id2).await.is_some());
        account2.delete(&mut db).await;
        assert!(Account::get(&mut db, &account_id2).await.is_none());

        // Clean up
        db.delete().await.unwrap();
    }
}
