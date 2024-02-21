use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rand::Rng;

fn generate_random_number(digits: usize) -> u32 {
    let mut rng = rand::thread_rng();
    let min_value = 10u32.pow(digits as u32 - 1);
    let max_value = 10u32.pow(digits as u32) - 1;
    rng.gen_range(min_value..=max_value)
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Random Number Generator! To generate a random number, add the desired number of digits to the URL, e.g., /5 \n\n\n
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
    ")
}

async fn random_number_handler(info: web::Path<(usize,)>) -> impl Responder {
    let digits = info.into_inner().0;
    let random_number = generate_random_number(digits);
    HttpResponse::Ok().body(format!(
        "Random number with {} digits / Numero Aleatorio de {} digitos : {} \n",
        digits, digits, random_number
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/{digits}").route(web::get().to(random_number_handler)))
    })
    // .bind("127.0.0.1:8080")?
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
