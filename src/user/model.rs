use crate::db::PgPool;
use crate::schema::users;
use crate::schema::users::dsl::*;
use crate::user::{model, request};

use actix_web::web;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct User {
  pub id: i32,
  pub name: String,
  pub level: Option<i32>,
  pub email: Option<String>,
  pub last_submitted: Option<String>,
}

impl User {
  pub fn get_all(pool: web::Data<PgPool>) -> QueryResult<Vec<User>> {
    let conn = &pool.get().unwrap();
    users::table.order(level.asc()).load::<User>(conn)
  }

  pub fn add(body: web::Json<request::UserRequest>, pool: web::Data<PgPool>) -> QueryResult<usize> {
    let conn = &pool.get().unwrap();

    let data = (
      &name.eq(&body.name),
      &level.eq(&body.level),
      &email.eq(&body.email),
      &last_submitted.eq(&body.last_submitted),
    );
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
      email: body.email.clone(),
      last_submitted: body.last_submitted.clone(),
    };
    diesel::update(users)
      .filter(name.eq(body.name.clone()))
      .set(data)
      .get_result::<model::User>(conn)
  }
}
