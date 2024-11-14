use reqwest::Error;
use serde::Deserialize;
// use std::env;

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let program = args[0]
//     for i in &args {
//         println!("{}", i);
//     }
//     println!("{:?}", args);
// }
#[derive(Deserialize, Debug)]
struct Planet {
    object_id: String,
    name: String,
    climate: Vec<String>,
    diameter: Option<u64>,
    films: Vec<String>,
    gravity: String,
    orbital_period: Option<u64>,
    population: Option<String>,
    residents: Vec<String>,
    rotation_period: Option<u64>,
    species: Vec<String>,
    surface_water: Option<f64>,
    terrain: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = "https://swapi.dev/api/planets/2/";
    let response = reqwest::get(request_url).await?.json::<Planet>().await?;

    println!("{:#?}", response);

    Ok(())
}
