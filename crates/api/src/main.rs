use rocket::{self, get, launch, routes, Build, Rocket};

#[launch]
fn rocket() -> Rocket<Build> {
  rocket::build().mount("/", routes![root])
}

#[get("/")]
fn root() -> Result<&'static str, &'static str> {
  Ok("&'static str")
}
