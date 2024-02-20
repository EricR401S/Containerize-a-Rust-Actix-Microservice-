use actix_web::{web, App, HttpServer, HttpResponse};
use lotto_randomizer::generate_random_number;

async fn index(digits: web::Path<usize>) -> HttpResponse {
    let random_number = generate_random_number(digits.into_inner());
    HttpResponse::Ok().body(format!("Random number: {}", random_number))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(web::resource("/{digits}").to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
