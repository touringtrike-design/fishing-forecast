### Stack
```
Frontend: React/Vue/Svelte (на ваш вибір)
Desktop: Tauri (Rust wrapper)
Backend: Той самий Rust Axum server
Database: PostgreSQL + PostGIS
```

### Чому Tauri:

- ✅ Rust backend "вбудований" в app
- ✅ Набагато менший розмір (3MB vs 150MB Electron)
- ✅ Нативна продуктивність
- ✅ Безпека Rust
- ✅ Можна використовувати будь-який web framework

**Мінуси:**
- ❌ Мобільна підтримка в beta (iOS/Android експериментальні)
- ❌ Складніше налаштувати

### Структура:
```
fishing-app/
├── src-tauri/           # Rust backend
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs      # Tauri entry
│       ├── commands.rs  # Rust функції для фронтенду
│       └── services/
│
├── src/                 # React/Vue/Svelte frontend
│   ├── App.tsx
│   ├── components/
│   └── services/
│
└── package.json
Приклад Rust commands для Tauri:
// src-tauri/src/commands.rs
use tauri::command;

#[command]
async fn get_forecast(lat: f64, lon: f64) -> Result<ForecastResponse, String> {
    let weather = fetch_weather(lat, lon)
        .await
        .map_err(|e| e.to_string())?;
    
    let prediction = predict_bite(&weather, lat, lon)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(ForecastResponse {
        bite_probability: prediction.probability,
        weather,
        recommended_bait: prediction.baits,
        best_time: prediction.best_time,
    })
}

// src-tauri/src/main.rs
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_forecast])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
Frontend (React):


// src/App.tsx
import { invoke } from '@tauri-apps/api/tauri';

async function getForecast(lat: number, lon: number) {
  const forecast = await invoke<ForecastResponse>('get_forecast', { 
    lat, 
    lon 
  });
  return forecast;
}
```
