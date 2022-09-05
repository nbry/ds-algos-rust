use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct WorkOrder {
    pub id: i32,
    pub work_code: String,
    pub done: bool,
}

fn practice_serde_json() {
    let mut work_vec = Vec::new();

    let work1 = WorkOrder {
        id: 1,
        work_code: "foo".to_string(),
        done: false,
    };
    work_vec.push(work1);

    let work2 = WorkOrder {
        id: 2,
        work_code: "foo".to_string(),
        done: false,
    };
    work_vec.push(work2);

    let json_str = serde_json::to_string(&work_vec).unwrap();
    println!("Serialize: {}", json_str);

    let work_from_json: Vec<WorkOrder> = serde_json::from_str(json_str.as_str()).unwrap();
    println!("Deserialize: {:?}", work_from_json);
}
