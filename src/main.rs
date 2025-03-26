use rocket::{launch, routes};
mod helpers;
mod models;
mod routes;
use routes::planets::{
    all_planets, planet_by_index, planets_by_population, planets_by_population_density,
    planets_by_size,
};

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/api",
        routes![
            all_planets,
            planet_by_index,
            planets_by_population,
            planets_by_size,
            planets_by_population_density,
        ],
    )
}
