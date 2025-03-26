# Star Wars Planets

- RESTful endpoints for querying planetary data
- Data enrichment with calculated properties (e.g., surface area, population density)
- Efficient pagination to fetch the complete dataset
- ️ Robust error handling for stability and reliability

All thanks to the [Star Wars API](https://swapi.dev/)!

## 🚀 Endpoints

### General

| Endpoint                | Description                |
| ----------------------- | -------------------------- |
| **`GET /planets`**      | Returns all planets        |
| **`GET /planets/<id>`** | Returns a planet by its ID |

### Sorting

| Endpoint                              | Description                                  |
| ------------------------------------- | -------------------------------------------- |
| **`GET /planets/population`**         | Returns planets sorted by population         |
| **`GET /planets/size`**               | Returns planets sorted by surface area       |
| **`GET /planets/population/density`** | Returns planets sorted by population density |

## 📔 Quickstart

### 1. Install Rust

If Rust isn't installed, use [rustup](https://rustup.rs/)

### 2. Clone the repo

```bash
git clone <repo>
cd <project-folder>
```

### 3. Build the project

```bash
cargo build
```

### 4. Run the server

```bash
cargo run
```

### 5. Do what you want with the data!

```rust
pub async fn fetch_planets_from_api() -> Result<Vec<Planet>, reqwest::Error> {
    let response = reqwest::get("http://127.0.0.1:8000/api/planets/size")
        .await?
        .json::<Vec<Planet>>()
        .await?;
    Ok(response)
}
```
