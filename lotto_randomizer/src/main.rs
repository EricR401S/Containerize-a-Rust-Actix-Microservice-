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
    "Welcome to the Random Number Generator! To generate a random number, add the desired number of digits to the URL, e.g., /5 \n\n\n
    Bienvenidos al generador aleatorio de numeros, para inspirar sus opciones de Pega 3 , Pega 4, o cualquier loteria. \n 
    Para obtener el numero deseado de digitos, anada la diagonal y el digito al enlace arriab (e.j. '/5')\n
    
    Utilizo esta plataforma tambien para mandar unos grandes saludos y abrazos a :\n
    
    Luis 'Tito' Rios\n
    Josephine Soderman\n

    Luis 'Nando' Rios Vargas\n
    Ileis Rios \n
    Ivelisse Carrion \n
    Ludy Soderman \n
    Nazim \n
    Yilmaz \n
    Fernando Lopez-Amill \n
    Quiro'curaespaldas'practico Erick David Cintron\n 
    "
}

async fn generate_random_number_handler(params: axum::extract::Path<(usize,)>) -> String {
    let digits = params.0 .0; // Extract the usize from the tuple
    let random_number = generate_random_number(digits);

    format!("Generated Random Number with {} digits / Numero aleatorio generado con {} digitos: {}\n", digits, digits, random_number)
}
