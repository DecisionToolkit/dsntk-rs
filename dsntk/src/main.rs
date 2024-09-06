//! # Decision Toolkit

mod actions;
mod examples;
/// Main entrypoint of the application.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
  actions::do_action().await
}
