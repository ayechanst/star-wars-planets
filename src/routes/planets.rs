use crate::{helpers, models::Planet};
use helpers::get_planets;
use rocket::{get, serde::json::Json};

#[get("/planets")]
pub async fn all_planets() -> Json<Vec<Planet>> {
    let planets = match get_planets().await {
        Ok(data) => data,
        Err(_) => Vec::new(),
    };
    Json(planets)
}

#[get("/planets/<id>")]
pub async fn planet_by_index(id: usize) -> Option<Json<Planet>> {
    let planets = match get_planets().await {
        Ok(data) => data,
        Err(_) => Vec::new(),
    };
    // planets.into_iter().nth(id).map(Json)
    planets.get(id).cloned().map(Json)
}
