use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Planet {
    pub name: String,
    pub rotation_period: String,
    pub orbital_period: String,
    pub diameter: String,
    pub surface_area: f32,
    pub comparison_factor: f32,
    pub climate: String,
    pub gravity: String,
    pub terrain: String,
    pub surface_water: String,
    pub population: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct PlanetsResponse {
    pub count: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<SwapiPlanet>,
}

#[derive(Deserialize, Debug)]
pub struct SwapiPlanet {
    pub name: String,
    pub rotation_period: String,
    pub orbital_period: String,
    pub diameter: String,
    pub climate: String,
    pub gravity: String,
    pub terrain: String,
    pub surface_water: String,
    pub population: String,
    pub residents: Vec<String>,
    pub films: Vec<String>,
    pub created: String,
    pub edited: String,
    pub url: String,
}
