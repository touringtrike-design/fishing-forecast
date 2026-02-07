Stack
Mobile/Web: Flutter (Dart)
Backend: Rust (Axum/Actix-web)
Database: PostgreSQL + PostGIS + SQLx
Карти: flutter_map + MapLibre
Cache: Redis
### Чому це ідеально:

**Flutter** замість Rust WASM:

- ✅ **Швидша розробка UI** (тижні замість місяців)
- ✅ Один код → iOS + Android + Web
- ✅ Багато готових компонентів для карт
- ✅ Hot reload працює чудово
- ✅ Dart простіший за Rust для UI

**Rust Backend** (ваша вимога):

- ✅ Вся бізнес-логіка на Rust
- ✅ Швидкість, надійність
- ✅ ML inference на Rust

### Час розробки MVP: **10-12 тижнів**

### Структура проекту:
fishing-app/
├── backend/              # Rust API
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── routes/
│   │   ├── services/
│   │   │   ├── weather.rs
│   │   │   ├── water_bodies.rs
│   │   │   ├── ml_predictor.rs
│   │   │   └── geo.rs
│   │   ├── models/
│   │   └── db/
│   └── migrations/
│
├── mobile/              # Flutter app
│   ├── pubspec.yaml
│   ├── lib/
│   │   ├── main.dart
│   │   ├── screens/
│   │   ├── widgets/
│   │   ├── services/
│   │   │   └── api_client.dart
│   │   └── models/
│   └── assets/
│
└── shared/              # Спільні схеми (опційно)
    └── api_spec.json    # OpenAPI spec

### Приклад інтеграції:

**Rust Backend:**
// backend/src/main.rs
use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};

#[derive(Debug, Deserialize)]
struct ForecastQuery {
    lat: f64,
    lon: f64,
}

#[derive(Debug, Serialize)]
struct ForecastResponse {
    bite_probability: f64,
    weather: WeatherData,
    recommended_bait: Vec<String>,
    best_time: String,
}

async fn get_forecast(
    Query(params): Query<ForecastQuery>,
    State(pool): State<PgPool>,
) -> Result<Json<ForecastResponse>, StatusCode> {
    // Отримати погоду
    let weather = fetch_weather(params.lat, params.lon)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // ML прогноз
    let prediction = predict_bite(&weather, params.lat, params.lon)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(ForecastResponse {
        bite_probability: prediction.probability,
        weather,
        recommended_bait: prediction.baits,
        best_time: prediction.best_time,
    }))
}

#[tokio::main]
async fn main() {
    let pool = PgPool::connect("postgres://...").await.unwrap();
    
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    let app = Router::new()
        .route("/api/forecast", get(get_forecast))
        .route("/api/spots/nearby", get(get_nearby_spots))
        .route("/api/catch", post(save_catch))
        .layer(cors)
        .with_state(pool);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    axum::serve(listener, app).await.unwrap();
}

Flutter Frontend:
dart
// mobile/lib/services/api_client.dart
import 'package:http/http.dart' as http;
import 'dart:convert';

class ApiClient {
  static const String baseUrl = 'http://localhost:3000/api';
  
  Future<ForecastResponse> getForecast(double lat, double lon) async {
    final response = await http.get(
      Uri.parse('$baseUrl/forecast?lat=$lat&lon=$lon'),
    );
    
    if (response.statusCode == 200) {
      return ForecastResponse.fromJson(jsonDecode(response.body));
    } else {
      throw Exception('Failed to load forecast');
    }
  }
}

// mobile/lib/models/forecast.dart
class ForecastResponse {
  final double biteProbability;
  final WeatherData weather;
  final List<String> recommendedBait;
  final String bestTime;
  
  ForecastResponse({
    required this.biteProbability,
    required this.weather,
    required this.recommendedBait,
    required this.bestTime,
  });
  
  factory ForecastResponse.fromJson(Map<String, dynamic> json) {
    return ForecastResponse(
      biteProbability: json['bite_probability'],
      weather: WeatherData.fromJson(json['weather']),
      recommendedBait: List<String>.from(json['recommended_bait']),
      bestTime: json['best_time'],
    );
  }
}

// mobile/lib/screens/map_screen.dart
import 'package:flutter/material.dart';
import 'package:flutter_map/flutter_map.dart';
import 'package:latlong2/latlong.dart';

class MapScreen extends StatefulWidget {
  @override
  _MapScreenState createState() => _MapScreenState();
}

class _MapScreenState extends State<MapScreen> {
  final ApiClient _api = ApiClient();
  ForecastResponse? _forecast;
  LatLng? _selectedPoint;
  
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Stack(
        children: [
          FlutterMap(
            options: MapOptions(
              center: LatLng(50.4501, 30.5234),
              zoom: 10.0,
              onTap: (tapPosition, point) async {
                setState(() {
                  _selectedPoint = point;
                  _forecast = null;
                });
                
                // Запит до Rust API
                final forecast = await _api.getForecast(
                  point.latitude,
                  point.longitude,
                );
                
                setState(() {
                  _forecast = forecast;
                });
              },
            ),
            children: [
              TileLayer(
                urlTemplate: 'https://tile.openstreetmap.org/{z}/{x}/{y}.png',
              ),
              if (_selectedPoint != null)
                MarkerLayer(
                  markers: [
                    Marker(
                      point: _selectedPoint!,
                      child: Icon(Icons.location_on, color: Colors.red, size: 40),
                    ),
                  ],
                ),
            ],
          ),
          if (_forecast != null)
            Positioned(
              bottom: 0,
              left: 0,
              right: 0,
              child: ForecastPanel(forecast: _forecast!),
            ),
        ],
      ),
    );
  }
}
```

---



