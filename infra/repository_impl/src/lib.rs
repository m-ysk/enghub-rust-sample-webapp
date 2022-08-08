use anyhow::{self, Context};
use async_trait::async_trait;
use derive_new::new;
use diesel::{
    pg::{upsert::excluded, PgConnection},
    prelude::*,
    r2d2::ConnectionManager,
    Insertable, Queryable,
};
use r2d2::Pool;

use db_schema::users;
use domain::{User, UserId, UserRepository};
use error::AppError;

#[derive(Queryable, Insertable)]
#[table_name = "users"]
struct UserRecord {
    pub id: String,
    pub name: String,
}

impl From<&User> for UserRecord {
    fn from(user: &User) -> UserRecord {
        UserRecord {
            id: user.id().to_string(),
            name: user.name().to_string(),
        }
    }
}

impl TryFrom<UserRecord> for User {
    type Error = anyhow::Error;

    fn try_from(user: UserRecord) -> anyhow::Result<User> {
        let UserRecord { id, name } = user;

        User::reconstruct(id, name)
    }
}

#[derive(new)]
pub struct UserRepositoryImpl {
    pool: Pool<ConnectionManager<PgConnection>>,
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn save(&self, user: &User) -> anyhow::Result<()> {
        tokio::task::block_in_place(|| {
            let user = UserRecord::from(user);
            let conn = self.pool.get()?;

            diesel::insert_into(users::table)
                .values(user)
                .on_conflict(users::id)
                .do_update()
                .set(users::name.eq(excluded(users::name)))
                .execute(&conn)
                .with_context(|| {
                    AppError::Internal("failed to insert or update user".to_string())
                })?;

            Ok(())
        })
    }

    async fn get_by_ids(&self, ids: &[UserId]) -> anyhow::Result<Vec<User>> {
        tokio::task::block_in_place(|| {
            let ids = ids.iter().map(|id| id.to_string()).collect::<Vec<_>>();
            let conn = self.pool.get()?;

            let users = users::table
                .filter(users::id.eq_any(ids))
                .load::<UserRecord>(&conn)
                .with_context(|| AppError::Internal("failed to get user".to_string()))?;

            users.into_iter().map(TryInto::try_into).collect()
        })
    }
}
