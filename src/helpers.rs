use crate::models::{self, Planet, PlanetsResponse, SwapiPlanet};
use reqwest::Error;
use std::collections::HashMap;

pub fn sort_population(planets: &mut Vec<Planet>) -> Vec<(String, u64)> {
    let mut sorted_populations = planets
        .iter()
        .filter_map(|planet| {
            planet
                .population
                .parse::<u64>()
                .ok()
                .map(|pop| (planet.name.clone(), pop))
        })
        .collect::<Vec<(String, u64)>>();
    sorted_populations.sort_by(|a, b| b.1.cmp(&a.1));
    sorted_populations
}

pub fn count_climates(planets: &Vec<Planet>) -> HashMap<String, u32> {
    let mut climate_hashmap = HashMap::new();
    for planet in planets {
        for climate in planet.climate.split(", ").map(|c| c.trim()) {
            *climate_hashmap.entry(climate.to_string()).or_insert(0) += 1;
        }
    }
    climate_hashmap
}

pub fn find_surface_area(planet_diameter: &String) -> f32 {
    const PI: f32 = 3.141592653589793;
    if planet_diameter == "unknown" {
        return 0.0;
    } else {
        let float_planet_diameter: f32 = planet_diameter
            .parse()
            .expect("Failed to parse String as Number");
        let surface_area = (float_planet_diameter * float_planet_diameter) * PI;
        surface_area
    }
}

pub fn surface_area_relative_to_earth(planet_surface_area: f32) -> f32 {
    const EARTH_SURFACE_AREA: f32 = 510000000.0;
    let comparison_factor = planet_surface_area / EARTH_SURFACE_AREA;
    comparison_factor
}

pub async fn get_planets() -> Result<Vec<Planet>, Error> {
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
