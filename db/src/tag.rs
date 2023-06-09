use crate::{new_id, DB};
use chrono::NaiveDateTime;

/// A representation of a tag on a transaction in the database.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tag {
    /// The tag's identifier.
    pub id: String,
    /// The name of the tag.
    pub name: String,
    /// A description of the tag.
    pub description: Option<String>,
    /// When the tag was created.
    pub created_at: NaiveDateTime,
}

impl Tag {
    /// Creates a new tag.
    pub async fn create(db: &mut DB, name: &str, description: &str) -> Self {
        let id = new_id();

        sqlx::query!(
            "INSERT INTO tag (id, name, description) VALUES (?, ?, ?);",
            id,
            name,
            description
        )
        .execute(&mut **db)
        .await
        .unwrap();

        Self::get(db, &id).await.unwrap()
    }

    /// Gets a tag from the database.
    pub async fn get(db: &mut DB, id: &str) -> Option<Self> {
        sqlx::query_as!(Self, "SELECT * FROM tag WHERE id = ?;", id)
            .fetch_optional(&mut **db)
            .await
            .unwrap()
    }

    /// Lists all tags in the database.
    pub async fn list(db: &mut DB) -> Vec<Self> {
        sqlx::query_as!(Self, "SELECT * FROM tag ORDER BY name;")
            .fetch_all(&mut **db)
            .await
            .unwrap()
    }

    /// Sets the tag name.
    pub async fn set_name(&mut self, db: &mut DB, name: &str) {
        self.name = name.to_owned();

        sqlx::query!("UPDATE tag SET name = ? WHERE id = ?;", self.name, self.id)
            .execute(&mut **db)
            .await
            .unwrap();
    }

    /// Sets the tag description.
    pub async fn set_description(&mut self, db: &mut DB, description: &str) {
        self.description = Some(description.to_owned());

        sqlx::query!(
            "UPDATE tag SET description = ? WHERE id = ?;",
            self.description,
            self.id
        )
        .execute(&mut **db)
        .await
        .unwrap();
    }

    /// Deletes the tag from the database.
    pub async fn delete(self, db: &mut DB) {
        sqlx::query!("DELETE FROM tag WHERE id = ?;", self.id)
            .execute(&mut **db)
            .await
            .unwrap();
    }
}

/// Tag tests.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::TestDB;

    #[tokio::test]
    async fn test_tag() {
        // Init
        let mut db = TestDB::new().await.unwrap();

        // Create
        let mut tag1 = Tag::create(&mut db, "Tag 1", "Tag 1 description").await;
        let tag2 = Tag::create(&mut db, "Tag 2", "").await;

        // Get
        let tag3 = Tag::get(&mut db, &tag1.id).await.unwrap();
        assert_eq!(tag3, tag1);
        let tag4 = Tag::get(&mut db, &tag2.id).await.unwrap();
        assert_eq!(tag4, tag2);
        assert!(Tag::get(&mut db, "").await.is_none());

        // List
        let tags = Tag::list(&mut db).await;
        assert_eq!(tags.len(), 2);
        let tag5 = tags.iter().find(|x| x.id == tag1.id).unwrap();
        assert_eq!(tag5, &tag1);
        let tag6 = tags.iter().find(|x| x.id == tag2.id).unwrap();
        assert_eq!(tag6, &tag2);

        // Set name
        tag1.set_name(&mut db, "Not tag 1").await;
        assert_eq!(&tag1.name, "Not tag 1");
        let tag7 = Tag::get(&mut db, &tag1.id).await.unwrap();
        assert_eq!(tag7, tag1);

        // Set description
        tag1.set_description(&mut db, "Not tag 1 description").await;
        assert_eq!(
            tag1.description.as_ref().unwrap().as_str(),
            "Not tag 1 description"
        );
        let tag8 = Tag::get(&mut db, &tag1.id).await.unwrap();
        assert_eq!(tag8, tag1);

        // Delete
        let tag_id1 = tag1.id.clone();
        assert!(Tag::get(&mut db, &tag_id1).await.is_some());
        tag1.delete(&mut db).await;
        assert!(Tag::get(&mut db, &tag_id1).await.is_none());
        let tag_id2 = tag2.id.clone();
        assert!(Tag::get(&mut db, &tag_id2).await.is_some());
        tag2.delete(&mut db).await;
        assert!(Tag::get(&mut db, &tag_id2).await.is_none());

        // Clean up
        db.delete().await.unwrap();
    }
}
