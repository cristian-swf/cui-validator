extern crate rocket;
extern crate serde;
extern crate serde_json;

use rocket::{get, launch, routes};
use rocket::http::Status;
use rocket::response::status;
use rocket::State;
use rocket::serde::json::{ Value, json};
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
    //return a JSON containing details about the API
    json!({
        "name": "CUI Validator API",
        "description": "Validate a Romanian company identification number (CUI)",
        "author": "Cristian L."
    })
}

#[get("/uptime")]
fn uptime(app_state: &State<AppState>) -> rocket::response::status::Custom<serde_json::Value> {
    //calculate uptime
    let uptime = calculate_uptime(&app_state.start_time);
    //return status
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

    //calculate number of chars
    let cui_len = cui.len();

    if cui_len < 4 || cui_len > 10 {
        return false;
    }

    //calculate the check digit and compare to the last digit of the cui
    let check_digit = cui.chars().last().unwrap();
    let cui = &cui[..cui_len - 1];
    let mut cui_padded = cui.to_owned();

    while cui_padded.len() != 9 {
        cui_padded = format!("0{}", cui_padded);
    }

    let digits: Vec<u32> = cui_padded
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let sum = digits[0] * 7 + digits[1] * 5 + digits[2] * 3 + digits[3] * 2 + digits[4] * 1 +
               digits[5] * 7 + digits[6] * 5 + digits[7] * 3 + digits[8] * 2;

    let rest = sum % 11;

    if rest == 10 {
        return check_digit == '0';
    }

    return check_digit == std::char::from_digit(rest, 10).unwrap();
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::{Status, ContentType};
    use rocket::local::blocking::Client;
    use rocket::serde::json::{ Value, json};

    #[test]
    fn test_about_route() {
        let client = Client::tracked(rocket()).unwrap();

        // Send a GET request to the /about route
        let response = client.get("/about").dispatch();

        // Assert that the response status is Status::Ok (200 OK)
        assert_eq!(response.status(), Status::Ok);

        // Assert the content type is JSON (or any other expected content type)
        assert_eq!(response.content_type(), Some(ContentType::JSON));

        let body_str = response.into_string().expect("Failed to read response body");
        let expected_response = json!({
            "name": "CUI Validator API",
            "description": "Validate a Romanian company identification number (CUI)",
            "author": "Cristian L."
        });

        let parsed_response: Value = serde_json::from_str(&body_str).expect("Failed to parse response JSON");

        assert_eq!(parsed_response, expected_response);
    }

    #[test]
    fn test_uptime_route() {
        let client = Client::tracked(rocket()).unwrap();

        // Send a GET request to the /uptime route
        let response = client.get("/uptime").dispatch();

        // Assert that the response status is Status::Ok
        assert_eq!(response.status(), Status::Ok);

        // Parse the response JSON
        let body_str = response.into_string().expect("Failed to read response body");
        let parsed_response: serde_json::Value =
            serde_json::from_str(&body_str).expect("Failed to parse response JSON");

        // Check if the "status" key exists and its value is "online"
        let status_value = parsed_response["status"].as_str();

        assert_eq!(status_value, Some("online"));
    }

    #[test]
    fn test_validate_route_valid_cui() {
        let client = Client::tracked(rocket()).unwrap();

        // Send a GET request to the /validate route with a valid CUI
        let response = client.get("/validate/33034700").dispatch();

        // Assert that the response status is Status::Ok (200 OK)
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));

        // Parse the response JSON
        let body_str = response.into_string().expect("Failed to read response body");
        let parsed_response: serde_json::Value =
            serde_json::from_str(&body_str).expect("Failed to parse response JSON");

        // Check if the response indicates a valid CUI
        assert_eq!(parsed_response["status"], "valid");
        assert_eq!(parsed_response["message"], "CUI is valid");
    }

    #[test]
    fn test_validate_route_invalid_cui() {
        let client = Client::tracked(rocket()).unwrap();

        // Send a GET request to the /validate route with a valid CUI
        let response = client.get("/validate/123456").dispatch();

        // Assert that the response status is Status::BadRequest (400 OK)
        assert_eq!(response.status(), Status::BadRequest);
        assert_eq!(response.content_type(), Some(ContentType::JSON));

        // Parse the response JSON
        let body_str = response.into_string().expect("Failed to read response body");
        let parsed_response: serde_json::Value =
            serde_json::from_str(&body_str).expect("Failed to parse response JSON");

        // Check if the response indicates a valid CUI
        assert_eq!(parsed_response["status"], "invalid");
        assert_eq!(parsed_response["message"], "Invalid CUI");
    }
}