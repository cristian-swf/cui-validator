extern crate rocket;
extern crate serde;
extern crate serde_json;

use rocket::{get, launch, routes};
use rocket::http::Status;
use rocket::response::status;
use rocket::State;
use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};
use std::time::SystemTime;

struct AppState {
    start_time: SystemTime,
}

#[launch]
fn rocket() -> _ {
    let start_time = SystemTime::now();
    rocket::build()
        .manage(AppState { start_time })
        .mount("/", routes![about, uptime, validate])
}

#[get("/about")]
fn about() -> Value {
    json!({
        "name": "CUI Validator API",
        "description": "Validate a Romanian company identification number (CUI)",
        "author": "Cristian L."
    })
}

#[get("/uptime")]
fn uptime(app_state: &State<AppState>) -> rocket::response::status::Custom<serde_json::Value> {
    let uptime = calculate_uptime(&app_state.start_time);
    let response_data = json!({
        "status": "online",
        "uptime_seconds": uptime,
    });
    status::Custom(Status::Ok, response_data)
}

fn calculate_uptime(start_time: &SystemTime) -> u64 {
    let now = SystemTime::now();
    let uptime = now.duration_since(*start_time).expect("Time went backwards").as_secs();
    uptime
}

#[get("/validate/<cui>")]
fn validate(cui: String) -> Result<rocket::response::status::Custom<serde_json::Value>, rocket::response::status::Custom<serde_json::Value>> {
    let is_valid = validate_cui(&cui);

    if is_valid {
        let response_data = json!({ "status": "valid", "message": "CUI is valid" });
        Ok(status::Custom(Status::Ok, response_data))
    } else {
        let response_data = json!({ "status": "invalid", "message": "Invalid CUI" });
        Err(status::Custom(Status::BadRequest, response_data))
    }
}

fn validate_cui(cui: &str) -> bool {
    if !cui.chars().all(|c| c.is_numeric()) {
        return false;
    }

    let cui_len = cui.len();
    if cui_len < 4 || cui_len > 10 {
        return false;
    }

    let cifra_control = cui.chars().last().unwrap();
    let cui = &cui[..cui_len - 1];
    let mut cui_padded = cui.to_owned();

    while cui_padded.len() != 9 {
        cui_padded = format!("0{}", cui_padded);
    }

    let digits: Vec<u32> = cui_padded
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let suma = digits[0] * 7 + digits[1] * 5 + digits[2] * 3 + digits[3] * 2 + digits[4] * 1 +
               digits[5] * 7 + digits[6] * 5 + digits[7] * 3 + digits[8] * 2;

    let rest = suma % 11;

    if rest == 10 {
        return cifra_control == '0';
    }

    return cifra_control == std::char::from_digit(rest, 10).unwrap();
}