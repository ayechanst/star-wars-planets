use crate::models::Planet;
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
