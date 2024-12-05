use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/env")]
async fn get_env() -> impl Responder {
    // Respond with the environment variables
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  

    // Start the HTTP server
    HttpServer::new(|| {
        App::new()
            .service(get_env) // Register the route
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
