use actix_web::HttpResponse;
use std::process::Command;

pub async fn migrate() -> HttpResponse {
  let cmd = Command::new("diesel")
    .args(["migration", "run"])
    .spawn()
    .expect("failed to execute process");
  HttpResponse::Ok().body(format!("{:?}", cmd.stdout))
}
