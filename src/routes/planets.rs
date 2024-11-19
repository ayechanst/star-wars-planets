use crate::{
    helpers::{self, sort_population},
    models::Planet,
};
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
    planets.get(id).cloned().map(Json)
}

#[get("/planets/population")]
pub async fn planets_by_population() -> Json<Vec<(String, u64)>> {
    let mut planets = match get_planets().await {
        Ok(data) => data,
        Err(_) => Vec::new(),
    };
    let sorted_planets = sort_population(&mut planets);
    rocket::serde::json::Json(sorted_planets)
}
