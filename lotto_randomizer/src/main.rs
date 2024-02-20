// use actix_web::{web, App, HttpServer, HttpResponse};
// use lotto_randomizer::generate_random_number;

// async fn index(digits: web::Path<usize>) -> HttpResponse {
//     let random_number = generate_random_number(digits.into_inner());
//     HttpResponse::Ok().body(format!("Random number: {}", random_number))
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new().service(web::resource("/{digits}").to(index))
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }

// src/main.rs

// use axum::{handler::get, Router};
// use lotto_randomizer::generate_random_number;

// #[tokio::main]
// async fn main() {
//     let app = Router::new().route("/", get(generate_random_number_handler));

//     axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }

// async fn generate_random_number_handler() -> &'static str {
//     let digits = 5; // Set the desired number of digits here
//     let random_number = generate_random_number(digits);

//     println!("Generated random number: {}", random_number);

//     "Check the server logs for the generated random number."
// }



// src/main.rs

// use axum::{routing::get, Router};
// use lotto_randomizer::generate_random_number;

// #[tokio::main]
// async fn main() {
//     let app = Router::new().route("/", get(generate_random_number_handler));

//     axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }

// async fn generate_random_number_handler() -> &'static str {
//     let digits = 5; // Set the desired number of digits here
//     let random_number = generate_random_number(digits);

//     println!("Generated random number: {}", random_number);

//     "Check the server logs for the generated random number."
// }


// // src/main.rs

// use axum::{routing::get, Router};
// use lotto_randomizer::generate_random_number;


// #[tokio::main]
// async fn main() {
//     let app = Router::new().route("/:digits", get(generate_random_number_handler));

//     axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }

// async fn generate_random_number_handler(params: axum::extract::Path<(usize,)>) -> String {
//     let digits = params.0 .0; // Extract the usize from the tuple
//     let random_number = generate_random_number(digits);

//     format!("Generated Random Number with {} digits: {}", digits, random_number)
// }


// src/main.rs

use axum::{routing::get, Router};
use lotto_randomizer::generate_random_number;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(welcome_handler)).route("/:digits", get(generate_random_number_handler));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn welcome_handler() -> &'static str {
    "Welcome to the Random Number Generator! To generate a random number, add the desired number of digits to the URL, e.g., /5"
}

async fn generate_random_number_handler(params: axum::extract::Path<(usize,)>) -> String {
    let digits = params.0 .0; // Extract the usize from the tuple
    let random_number = generate_random_number(digits);

    format!("Generated Random Number with {} digits: {}", digits, random_number)
}
