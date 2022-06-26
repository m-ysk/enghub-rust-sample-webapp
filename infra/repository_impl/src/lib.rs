use anyhow::{self, Context as _};
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
                .context("ユーザの保存に失敗しました")?;

            Ok(())
        })
    }

    async fn get_by_ids(&self, _ids: &[UserId]) -> anyhow::Result<Vec<User>> {
        todo!()
    }
}
