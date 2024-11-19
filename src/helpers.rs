use crate::models::{self, Planet, PlanetsResponse, SwapiPlanet};
use num2words::{Lang, Num2Words};
use reqwest::Error;
use std::collections::HashMap;

////////////////////////////// ORGANIZERS //////////////////////////////

pub fn sort_population(planets: &mut Vec<Planet>) {
    planets.sort_by(|a, b| {
        let pop_a = a.population.parse::<u64>().unwrap_or(0);
        let pop_b = b.population.parse::<u64>().unwrap_or(0);
        pop_b.cmp(&pop_a) // Sort descending
    });
}

pub fn sort_by_size(planets: &mut Vec<Planet>) {
    planets.sort_by(|a, b| {
        let size_a = a.surface_area as u64;
        let size_b = b.surface_area as u64;
        size_b.cmp(&size_a)
    })
}

pub fn sort_population_density(planets: &mut Vec<Planet>) {
    planets.sort_by(|a, b| {
        let pop_density_a = a.population_density as u64;
        let pop_density_b = b.population_density as u64;
        pop_density_b.cmp(&pop_density_a)
    });
}

////////////////////////////// HELPERS /////////////////////////////////////

pub fn find_population_density(surface_area: f32, population_string: &String) -> f32 {
    let parsed_population: Result<i64, _> = population_string.parse();
    match parsed_population {
        Ok(pop) => {
            if surface_area != 0.0 {
                pop as f32 / surface_area
            } else {
                f32::NAN
            }
        }
        Err(_) => f32::NAN,
    }
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

pub fn num2word(population: &String) -> String {
    let parsed_population: Result<i64, _> = population.parse();
    match parsed_population {
        Ok(pop) => Num2Words::new(pop as i64)
            .lang(Lang::English)
            .to_words()
            .unwrap_or_else(|_| "Conversion Error".to_string()),
        Err(_) => "Invalid Population Value".to_string(),
    }
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

//////////////////////////////// GETTER ///////////////////////////////////////

pub async fn get_planets() -> Result<Vec<Planet>, Error> {
    let mut planets = Vec::<Planet>::new();
    let mut next_url = Some("https://swapi.dev/api/planets".to_string());

    while let Some(url) = next_url {
        let response = reqwest::get(&url).await?.json::<PlanetsResponse>().await?;

        for swapi_planet in response.results {
            // Using helper functions to build more data
            let diameter = swapi_planet.diameter;
            let population = swapi_planet.population;
            let surface_area = find_surface_area(&diameter);
            let comparison_factor = surface_area_relative_to_earth(surface_area);
            let population_word = num2word(&population);
            let population_density = find_population_density(surface_area, &population);

            // Builds the Struct
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
                population: population,
                population_word: population_word,
                population_density: population_density,
                url: swapi_planet.url,
            };
            planets.push(simple_planet);
        }
        next_url = response.next;
    }
    Ok(planets)
}
