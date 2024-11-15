mod helpers;
mod models;
use helpers::{count_climates, find_surface_area, sort_population, surface_area_relative_to_earth};
use models::{Planet, PlanetsResponse};
use reqwest::Error;
// let response = reqwest::get(next_url).await?.text().await?;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut planets = get_planets().await?;
    // let climates = count_climates(&planets);
    let populations = sort_population(&mut planets);
    // println!("{:#?}", populations);
    println!("{:#?}", planets);
    // println!("{:#?}", climates);
    Ok(())
}

async fn get_planets() -> Result<Vec<Planet>, Error> {
    let mut planets = Vec::<Planet>::new();
    let mut next_url = Some("https://swapi.dev/api/planets".to_string());

    while let Some(url) = next_url {
        let response = reqwest::get(&url).await?.json::<PlanetsResponse>().await?;

        for swapi_planet in response.results {
            let diameter = swapi_planet.diameter;
            let surface_area = find_surface_area(&diameter);
            let comparison_factor = surface_area_relative_to_earth(surface_area);
            let simple_planet = Planet {
                name: swapi_planet.name,
                rotation_period: swapi_planet.rotation_period, // time taken to rotate around its poles
                orbital_period: swapi_planet.orbital_period,   // time taken to orbit it's star
                diameter: diameter,                            // kilometers
                surface_area: surface_area,
                comparison_factor: comparison_factor, // size compared to Earth
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
