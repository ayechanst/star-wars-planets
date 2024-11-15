use crate::models::Planet;
use std::collections::HashMap;

pub fn sort_population(planets: &mut Vec<Planet>) {
    planets.sort_by(|a, b| {
        a.population
            .parse::<u64>()
            .unwrap_or(0)
            .cmp(&b.population.parse::<u64>().unwrap_or(0))
    });
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
