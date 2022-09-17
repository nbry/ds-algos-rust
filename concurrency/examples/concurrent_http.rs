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

async fn get_attributes(poke_id: i32) -> Result<PokemonAttributes, Box<dyn std::error::Error>> {
    let url = format!("{}/pokemon/{}", POKE_API_BASE_URL, poke_id);

    let attributes = reqwest::Client::new()
        .request(Method::GET, url)
        .send()
        .await?
        .json::<PokemonAttributes>()
        .await?;

    println!("{:?}", attributes);

    Ok(attributes)
}

async fn serial() -> Result<(), Box<dyn std::error::Error>> {
    get_attributes(1).await?;
    get_attributes(2).await?;
    get_attributes(3).await?;
    get_attributes(4).await?;
    get_attributes(5).await?;
    get_attributes(6).await?;
    get_attributes(7).await?;
    get_attributes(8).await?;
    get_attributes(9).await?;
    get_attributes(10).await?;
    get_attributes(11).await?;
    get_attributes(12).await?;
    get_attributes(13).await?;
    get_attributes(14).await?;
    get_attributes(15).await?;
    get_attributes(16).await?;
    get_attributes(17).await?;
    get_attributes(18).await?;
    get_attributes(19).await?;
    get_attributes(20).await?;
    get_attributes(21).await?;
    get_attributes(22).await?;
    get_attributes(23).await?;
    get_attributes(24).await?;
    get_attributes(25).await?;
    get_attributes(26).await?;
    get_attributes(27).await?;
    get_attributes(28).await?;
    get_attributes(29).await?;
    get_attributes(30).await?;

    Ok(())
}

async fn concurrent() -> Result<(), Box<dyn std::error::Error>> {
    let _ = tokio::join!(
        get_attributes(1),
        get_attributes(2),
        get_attributes(3),
        get_attributes(4),
        get_attributes(5),
        get_attributes(6),
        get_attributes(7),
        get_attributes(8),
        get_attributes(9),
        get_attributes(10),
        get_attributes(11),
        get_attributes(12),
        get_attributes(13),
        get_attributes(14),
        get_attributes(15),
        get_attributes(16),
        get_attributes(17),
        get_attributes(18),
        get_attributes(19),
        get_attributes(20),
        get_attributes(21),
        get_attributes(22),
        get_attributes(23),
        get_attributes(24),
        get_attributes(25),
        get_attributes(26),
        get_attributes(27),
        get_attributes(28),
        get_attributes(29),
        get_attributes(30),
    );

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // concurrent().await?;
    // serial().await?;

    Ok(())
}
