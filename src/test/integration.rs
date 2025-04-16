#[cfg(test)]
mod tests {
    use super::super::{db, error::AppError, operations};
    use diesel::prelude::*;

    #[test]
    fn test_create_user_and_post() -> Result<(), AppError> {
        let mut conn = db::establish_connection();

        // Create a user
        let user = operations::create_user(&mut conn, "test_user", "test@example.com")?;
        assert_eq!(user.username, "test_user");

        // Create a post
        operations::create_post(&mut conn, user.id, "Test Post", "This is a test.", true)?;

        // Query posts
        let results = operations::get_posts_with_users(&mut conn)?;
        assert_eq!(results.len(), 1);
        let (post, queried_user) = &results[0];
        assert_eq!(post.title, "Test Post");
        assert_eq!(queried_user.username, "test_user");

        Ok(())
    }
}
