use crate::models::{Planet, SwapiPlanet};
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
