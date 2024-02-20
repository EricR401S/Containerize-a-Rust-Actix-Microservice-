// main.rs

// use random_yg_card::{print_card, random_card, read_cards_from_csv};

// fn main() {
//     // Specify the path to your CSV file
//     //println!("Current working directory: {:?}", std::env::current_dir());
//     let file_path = "data/Yugi_db_cleaned.csv";

//     // Read card data from the CSV file
//     let cards = match read_cards_from_csv(file_path) {
//         Ok(cards) => cards,
//         Err(err) => {
//             eprintln!("Error reading CSV file: {}", err);
//             return;
//         }
//     };

//     // Get a random card
//     if let Some(random_card) = random_card(&cards) {
//         // Print the contents of the random card
//         print_card(random_card);
//     } else {
//         println!("No cards available in the dataset.");
//     }
// }

// main.rs


// use random_yg_card::get_random_card;
// use actix_web::{web, App, HttpServer};

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(web::resource("/api").route(web::get().to(get_random_card)))
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }


// main.rs

use actix_web::{get, web, App, HttpResponse, HttpServer};
use random_yg_card::{read_cards_from_csv, random_card, print_card, Card};

#[derive(Debug)]
struct AppState {
    cards: Vec<Card>,
}

// Actix web handler to get a random card
#[get("/api/get_random_card")]
async fn get_random_card(data: web::Data<AppState>) -> HttpResponse {
    // Get a random card
    if let Some(random_card) = random_card(&data.cards) {
        // Print the contents of the random card (optional)
        print_card(random_card);

        // Return the card details as JSON
        HttpResponse::Ok().json(random_card)
    } else {
        HttpResponse::NotFound().body("No cards available in the dataset.")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Specify the path to your CSV file
    let file_path = "data/Yugi_db_cleaned.csv";

    // Read card data from the CSV file
    let cards = match read_cards_from_csv(file_path) {
        Ok(cards) => cards,
        Err(err) => {
            eprintln!("Error reading CSV file: {}", err);
            return Ok(());
        }
    };

    // Create Actix web app state with card data
    let app_state = web::Data::new(AppState { cards });

    // Start Actix web server
    HttpServer::new(move || {
        App::new().app_data(app_state.clone()).service(get_random_card)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
