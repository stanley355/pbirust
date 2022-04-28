use crate::db::PgPool;
use crate::schema::users;
use crate::schema::users::dsl::*;
use crate::user::{model, request};

use actix_web::web;
use diesel::{ExpressionMethods, QueryResult, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct User {
  pub id: i32,
  pub name: String,
  pub level: Option<i32>,
}

impl User {
  pub fn get_all(pool: web::Data<PgPool>) -> QueryResult<Vec<User>> {
    let conn = &pool.get().unwrap();
    users::table.load::<User>(conn)
  }

  pub fn add(body: web::Json<request::UserRequest>, pool: web::Data<PgPool>) -> QueryResult<usize> {
    let conn = &pool.get().unwrap();

    let data = (&name.eq(&body.name), &level.eq(&body.level));
    diesel::insert_into(users).values(data).execute(conn)
  }

  pub fn update(
    body: web::Json<request::UserRequest>,
    pool: web::Data<PgPool>,
  ) -> QueryResult<model::User> {
    let conn = &pool.get().unwrap();

    let data = request::UserRequest {
      name: body.name.clone(),
      level: body.level.clone(),
    };
    diesel::update(users)
      .filter(name.eq(body.name.clone()))
      .set(data)
      .get_result::<model::User>(conn)
  }
}
