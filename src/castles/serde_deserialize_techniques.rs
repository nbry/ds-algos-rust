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
enum Fruit {
    Apple(Apple),
    Watermelon(Watermelon),
    Orange(Orange),
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

    if let Fruit::Apple(a) = apple {
        println!("========================");
        println!("Apple struct:\n");
        println!("{:#?}", a);
    };
}

pub(crate) fn deserialize_watermelon() {
    let watermelon_json = json!( {
        "size": "large",
        "available": false,
    });

    // Serialize by untagged Fruit Enum
    let watermelon: Fruit = serde_json::from_str(watermelon_json.to_string().as_str()).unwrap();
    println!("========================");
    println!("Fruit::Apple enum:\n");
    println!("{:#?}", watermelon);

    if let Fruit::Watermelon(w) = watermelon {
        println!("========================");
        println!("Watermelon struct:\n");
        println!("{:#?}", w);
    };
}

pub(crate) fn deserialize_orange() {
    let orange_json = json!({
        "color": "uhmmm orange?",
        "spoiled": false,
        "meta" : {
            "something": "random",
            "somethingElse": "blah"
        }
    });

    // Serialize by untagged Fruit Enum
    let orange: Fruit = serde_json::from_str(orange_json.to_string().as_str()).unwrap();

    if let Fruit::Orange(o) = orange {
        println!("========================");
        println!("orange struct, with meta:\n");
        println!("{:#?}", o);
    };
}

pub(crate) fn deserialize_orange_without_meta() {
    let orange_json = json!( {
        "color": "uhmmm orange?",
        "spoiled": false,
    });

    // Serialize by untagged Fruit Enum
    let orange: Fruit = serde_json::from_str(orange_json.to_string().as_str()).unwrap();

    if let Fruit::Orange(o) = orange {
        println!("========================");
        println!("orange struct, without meta:\n");
        println!("{:#?}", o);
    };
}
