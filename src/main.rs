#[macro_use]
extern crate rocket;
mod helpers;
mod models;
mod routes;
use helpers::{count_climates, find_surface_area, sort_population, surface_area_relative_to_earth};
use models::{Planet, PlanetsResponse};
use reqwest::Error;
use rocket::serde::json::Json;
use routes::all_planets;

// let response = reqwest::get(next_url).await?.text().await?;

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     let mut planets = get_planets().await?;
//     // let climates = count_climates(&planets);
//     let populations = sort_population(&mut planets);
//     // println!("{:#?}", populations);
//     println!("{:#?}", planets);
//     // println!("{:#?}", climates);
//     Ok(())
// }

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![all_planets])
}
