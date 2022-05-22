use anyhow::{self, Context as _};
use async_trait::async_trait;
use diesel::{
    pg::{upsert::excluded, PgConnection},
    prelude::*,
    Insertable, Queryable,
};

use db_schema::users;
use domain::{User, UserId, UserRepository};

#[derive(Queryable, Insertable)]
#[table_name = "users"]
struct UserRecord {
    pub id: String,
    pub name: String,
}

impl From<User> for UserRecord {
    fn from(user: User) -> UserRecord {
        UserRecord {
            id: user.id().to_string(),
            name: user.name().to_string(),
        }
    }
}

pub struct UserRepositoryImpl {
    database_url: String,
}

impl UserRepositoryImpl {
    fn establish_connection(&self) -> anyhow::Result<PgConnection> {
        PgConnection::establish(&self.database_url).context("データベースへの接続に失敗しました")
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn save(&self, user: User) -> anyhow::Result<()> {
        tokio::task::block_in_place(|| {
            let user = UserRecord::from(user);
            let conn = self.establish_connection()?;

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
