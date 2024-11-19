use crate::{
    helpers::{self, sort_population},
    models::Planet,
};
use helpers::get_planets;
use rocket::{get, serde::json::Json};

async fn fetch_planets() -> Vec<Planet> {
    get_planets().await.unwrap_or_default()
}

#[get("/planets")]
pub async fn all_planets() -> Json<Vec<Planet>> {
    let planets = fetch_planets().await;
    Json(planets)
}

#[get("/planets/<id>")]
pub async fn planet_by_index(id: usize) -> Option<Json<Planet>> {
    let planets = fetch_planets().await;
    planets.get(id).cloned().map(Json)
}

#[get("/planets/population")]
pub async fn planets_by_population() -> Json<Vec<Planet>> {
    let mut planets = fetch_planets().await;
    sort_population(&mut planets);
    Json(planets)
}
