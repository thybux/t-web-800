use serde_json::Value;

#[macro_use]
extern crate rocket;
extern crate common;
use common::google::nearly_place_model::exploit_json;
use common::google::Google;

// URL pour récupérer les bar dans un périmétre donné et pour une localisation donnée
#[get("/service/drink/<localisation>/<radius>")]
async fn index(localisation: String, radius: i32) -> String {
    let bar = get_google(localisation, radius).await;
    let result = exploit_json(bar).unwrap();
    format!("{:?}", result)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

pub(crate) async fn get_google(localisation: String, radius: i32) -> Value {
    let mut google = Google::new();
    google
        .geocoding(localisation)
        .await
        .expect("Geocoding n'a pas été éxécuté correctement");
    let resto: Value = google
        .nearby_place(String::from("bar"), radius)
        .await
        .expect("FDP");
    resto
}


