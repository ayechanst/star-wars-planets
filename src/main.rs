use rocket::{launch, routes};
mod helpers;
mod models;
mod routes;
use routes::planets::{all_planets, planet_by_index, planets_by_population};

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
    rocket::build().mount(
        "/api",
        routes![all_planets, planet_by_index, planets_by_population],
    )
}
