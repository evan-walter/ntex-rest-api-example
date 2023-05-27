use ntex::web;

mod services;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::server(|| {
        web::App::new().default_service(web::route().to(services::default))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;
    Ok(())
}
