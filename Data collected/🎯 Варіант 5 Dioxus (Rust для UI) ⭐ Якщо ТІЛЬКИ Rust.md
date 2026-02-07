### Stack
```
Frontend: Dioxus (Rust UI framework)
Backend: Axum (Rust)
Database: PostgreSQL + PostGIS
Платформи: Web, Desktop, Mobile (всі з одного коду!)
### Чому Dioxus:

**Dioxus** - це як React, але на Rust:

- ✅ Компілюється в Web (WASM), Desktop (native), Mobile (work in progress)
- ✅ JSX-like синтаксис (RSX)
- ✅ Компонентна архітектура
- ✅ Hot reload
- ✅ **НАБАГАТО простіший за Leptos**
  Приклад коду:
  // Cargo.toml
[dependencies]
dioxus = "0.5"
dioxus-web = "0.5"
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }

// src/main.rs
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct ForecastData {
    bite_probability: f64,
    weather: WeatherData,
    recommended_bait: Vec<String>,
}

fn App() -> Element {
    let mut forecast = use_signal(|| None::<ForecastData>);
    let mut selected_lat = use_signal(|| 50.4501);
    let mut selected_lon = use_signal(|| 30.5234);
    
    // Асинхронний запит до API
    let get_forecast = move |_| async move {
        let url = format!(
            "http://localhost:3000/api/forecast?lat={}&lon={}",
            selected_lat(),
            selected_lon()
        );
        
        if let Ok(response) = reqwest::get(&url).await {
            if let Ok(data) = response.json::<ForecastData>().await {
                forecast.set(Some(data));
            }
        }
    };
    
    rsx! {
        div { class: "app-container",
            div { class: "map-section",
                h2 { "Оберіть точку на карті" }
                
                // Карта (можна інтегрувати через iframe або JS interop)
                div { 
                    id: "map",
                    style: "width: 100%; height: 500px;"
                }
                
                button {
                    onclick: get_forecast,
                    "Отримати прогноз"
                }
            }
            
            // Відображення прогнозу
            if let Some(ref data) = *forecast.read() {
                div { class: "forecast-panel",
                    h3 { "Прогноз клювання" }
                    p { "Ймовірність: {data.bite_probability * 100.0:.1}%" }
                    
                    div { class: "weather",
                        h4 { "Погода" }
                        p { "Температура: {data.weather.temperature}°C" }
                        p { "Тиск: {data.weather.pressure} мм" }
                    }
                    
                    div { class: "bait-recommendations",
                        h4 { "Рекомендовані наживки:" }
                        ul {
                            for bait in &data.recommended_bait {
                                li { "{bait}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    dioxus_web::launch(App);
}
```

### Структура проекту:
```
fishing-app/
├── backend/
│   └── (Rust Axum API як раніше)
│
├── frontend/            # Dioxus app
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── components/
│   │   │   ├── map.rs
│   │   │   ├── forecast_panel.rs
│   │   │   └── weather_widget.rs
│   │   ├── services/
│   │   │   └── api.rs
│   │   └── models/
│   │       └── forecast.rs
│   └── assets/
│
└── shared/              # Спільні типи
    └── Cargo.toml
    
    Інтеграція з картами:
    // src/components/map.rs
use dioxus::prelude::*;

#[component]
fn MapComponent(on_location_selected: EventHandler<(f64, f64)>) -> Element {
    use_effect(move || {
        // Ініціалізація MapLibre через JS interop
        let script = r#"
            const map = new maplibregl.Map({
                container: 'map',
                style: 'https://demotiles.maplibre.org/style.json',
                center: [30.5234, 50.4501],
                zoom: 10
            });
            
            map.on('click', (e) => {
                const coords = e.lngLat;
                // Викликати Rust callback
                window.onMapClick(coords.lat, coords.lng);
            });
        "#;
        
        eval(&script);
    });
    
    rsx! {
        div {
            id: "map",
            style: "width: 100%; height: 100%;"
        }
    }
}
**Плюси Dioxus:**

- ✅ Весь код на Rust
- ✅ Простіший за Leptos/Yew
- ✅ Одна кодова база для всіх платформ
- ✅ Хороша документація