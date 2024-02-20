// lib.rs

use csv::ReaderBuilder;
use rand::Rng;
use std::error::Error;
use std::fs::File;
//use std::path::Path;
// use actix_web::{get, web, App, HttpResponse, HttpServer};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Card {
    pub set: String,
    pub number: String,
    pub rarity: String,
    pub name: String,
    pub card_type: String,
    pub attribute: String,
    pub types: String,
    pub level: String,
    pub atk_def: String,
    pub ritual_required: String,
    pub property: String,
    pub ritual_monster_required: String,
    pub pendulum_scale: String,
    pub rank: String,
}

// Create a function that reads card data from the CSV file
pub fn read_cards_from_csv(file_path: &str) -> Result<Vec<Card>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    let mut cards = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let set = record.get(1).ok_or("Missing Card-set field")?.to_string();
        let number = record
            .get(2)
            .ok_or("Missing Card_number field")?
            .to_string();
        let rarity = record.get(3).ok_or("Missing Rarity field")?.to_string();
        let name = record.get(4).ok_or("Missing Card_name field")?.to_string();
        let card_type = record.get(6).ok_or("Missing Card type field")?.to_string();
        let attribute = record.get(7).ok_or("Missing Attribute field")?.to_string();
        let types = record.get(8).ok_or("Missing Types field")?.to_string();
        let level = record.get(9).ok_or("Missing Level field")?.to_string();
        let atk_def = record.get(10).ok_or("Missing ATK / DEF field")?.to_string();
        let ritual_required = record
            .get(13)
            .ok_or("Missing Ritual required field")?
            .to_string();
        let property = record.get(14).ok_or("Missing Property field")?.to_string();
        let ritual_monster_required = record
            .get(15)
            .ok_or("Missing Ritual Monster required field")?
            .to_string();
        let pendulum_scale = record
            .get(16)
            .ok_or("Missing Pendulum Scale field")?
            .to_string();
        let rank = record.get(17).ok_or("Missing Rank field")?.to_string();

        let card = Card {
            set,
            number,
            rarity,
            name,
            card_type,
            attribute,
            types,
            level,
            atk_def,
            ritual_required,
            property,
            ritual_monster_required,
            pendulum_scale,
            rank,
        };

        cards.push(card);
    }

    Ok(cards)
}

// Create a function that returns a random card from the dataset
pub fn random_card(cards: &[Card]) -> Option<&Card> {
    if cards.is_empty() {
        None
    } else {
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..cards.len());
        Some(&cards[random_index])
    }
}

// Create a function to print the contents of a card
pub fn print_card(card: &Card) {
    println!("Card-set: {}", card.set);
    println!("Card_number: {}", card.number);
    println!("Rarity: {}", card.rarity);
    println!("Card_name: {}", card.name);
    println!("Card type: {}", card.card_type);
    println!("Attribute: {}", card.attribute);
    println!("Types: {}", card.types);
    println!("Level: {}", card.level);
    println!("ATK / DEF: {}", card.atk_def);
    println!("Ritual required: {}", card.ritual_required);
    println!("Property: {}", card.property);
    println!("Ritual Monster required: {}", card.ritual_monster_required);
    println!("Pendulum Scale: {}", card.pendulum_scale);
    println!("Rank: {}", card.rank);
}

// // Actix web handler to get a random card
// #[get("/get_random_card")]
// async fn get_random_card() -> HttpResponse {
//     // Specify the path to your CSV file
//     println!("Current working directory: {:?}", std::env::current_dir());
//     let file_path = "data/Yugi_db_cleaned.csv";

//     // Read card data from the CSV file
//     let cards = match read_cards_from_csv(file_path) {
//         Ok(cards) => cards,
//         Err(err) => {
//             eprintln!("Error reading CSV file: {}", err);
//             return HttpResponse::InternalServerError().finish();
//         }
//     };

//     // Get a random card
//     if let Some(random_card) = random_card(&cards) {
//         // Print the contents of the random card (optional)
//         print_card(random_card);

//         // Return the card details as JSON
//         HttpResponse::Ok().json(random_card)
//     } else {
//         HttpResponse::NotFound().body("No cards available in the dataset.")
//     }
// }
