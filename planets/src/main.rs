mod helpers;
mod models;
use helpers::count_climates;
use models::{Planet, PlanetsResponse};
use reqwest::Error;
// let response = reqwest::get(next_url).await?.text().await?;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let planets = get_planets().await?;
    let climates = count_climates(&planets);
    // println!("{:#?}", planets);
    println!("{:#?}", climates);
    Ok(())
}

async fn get_planets() -> Result<Vec<Planet>, Error> {
    let mut planets = Vec::<Planet>::new();
    let mut next_url = Some("https://swapi.dev/api/planets".to_string());

    while let Some(url) = next_url {
        let response = reqwest::get(&url).await?.json::<PlanetsResponse>().await?;

        for swapi_planet in response.results {
            let simple_planet = Planet {
                name: swapi_planet.name,
                rotation_period: swapi_planet.rotation_period, // time taken to rotate around its poles
                orbital_period: swapi_planet.orbital_period,   // time taken to orbit it's star
                diameter: swapi_planet.diameter,
                climate: swapi_planet.climate,
                gravity: swapi_planet.gravity,
                terrain: swapi_planet.terrain,
                surface_water: swapi_planet.surface_water,
                population: swapi_planet.population,
                url: swapi_planet.url,
            };
            planets.push(simple_planet);
        }
        next_url = response.next;
    }
    Ok(planets)
}
