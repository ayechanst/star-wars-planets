# Star Wars Planets

- 🚀 RESTful endpoints for querying planetary data
- 📊 Data enrichment with calculated properties (e.g., surface area, population density)
- 🔄 Efficient pagination to fetch the complete dataset
- 🛡️ Robust error handling for stability and reliability

All thanks to the [Star Wars API](https://swapi.dev/)!

<!-- Here are the available endpoints:

| Endpoint | Description |
| -------- | -------------------- |
| GET /planets | Returns planets |
| GET /planets<id> | Returns a planets at <id> |
| GET /planets/population | Returns planets sorted by population |
| GET /planets/size | Returns planets sorted by size |
| GET /planets/population/density | Returns planets sorted by population density | -->

## 🚀 Endpoints

### 🌌 General

| Endpoint                | Description                |
| ----------------------- | -------------------------- |
| **`GET /planets`**      | Returns all planets        |
| **`GET /planets/<id>`** | Returns a planet by its ID |

### 📊 Sorting

| Endpoint                              | Description                                  |
| ------------------------------------- | -------------------------------------------- |
| **`GET /planets/population`**         | Returns planets sorted by population         |
| **`GET /planets/size`**               | Returns planets sorted by surface area       |
| **`GET /planets/population/density`** | Returns planets sorted by population density |

### Example

```rust
pub async fn fetch_planets_from_api() -> Result<Vec<Planet>, reqwest::Error> {
    let response = reqwest::get("http://127.0.0.1:8000/api/planets/size")
        .await?
        .json::<Vec<Planet>>()
        .await?;
    Ok(response)
}
```
