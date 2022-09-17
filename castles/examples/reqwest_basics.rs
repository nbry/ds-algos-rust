#![allow(dead_code)]

use reqwest::Method;

use serde::{Deserialize, Serialize};

const POKE_API_BASE_URL: &str = "https://pokeapi.co/api/v2";

#[derive(Deserialize, Serialize, Debug)]
struct PokemonAttributes {
    name: String,
    height: i32,
    weight: i32,
    abilities: Vec<Ability>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Ability {
    ability: AbilityDetails,
    is_hidden: bool,
    slot: i32,
}

#[derive(Deserialize, Serialize, Debug)]
struct AbilityDetails {
    name: String,
    url: String,
}

pub async fn print_pokemon_attributes() -> Result<(), Box<dyn std::error::Error>> {
    let mut pokemon_attributes = Vec::new();
    let client = reqwest::Client::new();
    println!("happening");

    for n in 1..5 {
        let poke_url = format!("{}/pokemon/{}", POKE_API_BASE_URL, n);

        let poke_response = client
            .request(Method::GET, poke_url)
            .send()
            .await?
            .json::<PokemonAttributes>()
            .await;

        pokemon_attributes.push(poke_response);
    }

    // println!("{:#?}", pokemon_attributes);
    Ok(())
}

fn main() {}
