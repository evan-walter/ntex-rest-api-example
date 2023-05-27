use ntex::web;

#[web::get("/")]
async fn index() -> &'static str {
    "Hello world!"
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::server(|| web::App::new().service(index))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await?;
    Ok(())
}
