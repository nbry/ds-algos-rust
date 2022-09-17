#![allow(dead_code)]
use std::collections::HashMap;

use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
pub struct Apple {
    name: String,
    color: String,
    quantity: i32,
}

#[derive(Deserialize, Debug)]
pub struct Watermelon {
    size: String,
    available: bool,
}

#[derive(Deserialize, Debug)]
pub struct Orange {
    color: String,
    spoiled: bool,
    meta: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Citrus {
    Or(Orange),
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum NonCitrus {
    Ap(Apple),
    Wm(Watermelon),
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Fruit {
    Ct(Citrus),
    Nc(NonCitrus),
    // Apple(Apple),
    // Watermelon(Watermelon),
    // Orange(Orange),
}

pub(crate) fn deserialize_apple() {
    let apple_json = json!( {
        "name": "granny smith",
        "color": "red",
        "quantity": 9
    });

    // Serialize by untagged Fruit Enum
    let apple: Fruit = serde_json::from_str(apple_json.to_string().as_str()).unwrap();
    println!("========================");
    println!("Fruit::Apple enum:\n");
    println!("{:#?}", apple);

    if let Fruit::Nc(nc) = apple {
        if let NonCitrus::Ap(a) = nc {
            println!("========================");
            println!("Apple Struct:\n");
            println!("{:#?}", a);
        }
    };
}

fn main() {}
