# 🎣 Fishing Forecast (Прогноз клювання)

> Your intelligent fishing companion - AI-powered fishing forecast application with multi-regional European support

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

## 🌟 Features

- **AI-Powered Predictions**: Multi-factor algorithm considering weather, moon phases, and historical data
- **Interactive Maps**: MapLibre GL with water bodies from OpenStreetMap
- **Safety First**: Emergency SOS button and weather warnings
- **Offline Mode**: Works without internet connection
- **Multi-Language**: Ukrainian, English, Polish, German, French
- **Regional Support**: Local fish species, baits, and fishing regulations
- **Catch Journal**: Log your catches with photos and conditions
- **Zero Cost**: Entirely hosted on free tiers (Shuttle.rs + Cloudflare Pages + Neon.tech)

## 🏗️ Architecture

Full Rust stack for maximum performance and type safety:

- **Frontend**: Dioxus + WASM (React-like ergonomics)
- **Backend**: Axum + Tokio (Async web framework)
- **Database**: PostgreSQL 15 + PostGIS (Geospatial data)
- **Deployment**: Shuttle.rs + Cloudflare Pages + Neon.tech

```
fishing-forecast/
├── crates/
│   ├── frontend/    # Dioxus web application
│   ├── backend/     # Axum API server
│   ├── shared/      # Common types and utilities
│   └── ml-engine/   # Future: ML prediction engine
├── docs/            # Documentation
└── .github/         # CI/CD workflows
```

## 🚀 Quick Start

### Prerequisites

- Rust 1.75+ ([Install](https://rustup.rs/))
- PostgreSQL 15+ with PostGIS ([Install](https://www.postgresql.org/download/)) - for production
- SQLite 3+ - for local development
- Trunk (`cargo install trunk`) - for frontend bundling
- Node.js 18+ - for Trunk asset pipeline

### Development Setup

```bash
# 1. Clone and setup
git clone https://github.com/your-username/fishing-forecast.git
cd fishing-forecast

# 2. Install Rust WASM target
rustup target add wasm32-unknown-unknown

# 3. Install frontend build tools
cargo install trunk
```

### Running Local Development

**Terminal 1 - Frontend (Port 3001):**
```bash
cd crates/frontend
trunk serve --address 127.0.0.1 --port 3001
```

**Terminal 2 - Backend (Port 8080):**
```bash
cd crates/backend
cargo run
```

Open browser: **http://127.0.0.1:3001**

### Database

Default configuration uses **SQLite** for development:
```bash
# Auto-created at: fishing.db
DATABASE_URL=sqlite:fishing.db
```

For PostgreSQL production:
```bash
DATABASE_URL=postgresql://user:password@localhost/fishing_db
```

## 📡 API Endpoints (Backend)

```bash
GET    /api/v1/health                    # Health check
GET    /api/v1/forecast?lat=50&lon=30    # Get bite forecast
GET    /api/v1/fish?country=UA           # List fish species
GET    /api/v1/water-bodies?lat=50&lon=30&radius_km=20
GET    /api/v1/regulations?country_code=UA
GET    /api/v1/region/detect?lat=50&lon=30

POST   /api/v1/catches                   # Save catch record
GET    /api/v1/catches                   # Get user catches
GET    /api/v1/catches/nearby?lat=50&lon=30

POST   /api/v1/regulations/validate      # Validate catch legality
```

## 🧪 Testing & Building

### Frontend
```bash
cd crates/frontend

# Watch + rebuild on changes
trunk serve --address 127.0.0.1 --port 3001

# Production build
trunk build --release

# Type checking
cargo check --target wasm32-unknown-unknown
```

### Backend
```bash
cd crates/backend

# Run with hot reload (requires cargo-watch)
cargo install cargo-watch
cargo watch -x run

# Tests
cargo test

# Linting
cargo clippy
cargo fmt --check
```

## 📦 Production Setup

1. **Build optimized binaries:**
   ```bash
   # Frontend
   cd crates/frontend && trunk build --release
   # Outputs: dist/

   # Backend
   cd crates/backend && cargo build --release
   # Outputs: target/release/fishing-backend
   ```

2. **Deploy frontend (static WASM):**
   - Upload `dist/` folder to Cloudflare Pages or similar
   - Set environment variables for backend URL

3. **Deploy backend:**
   - Use PostgreSQL instead of SQLite
   - Run migrations: `sqlx migrate run`
   - Set production environment variables
   - Deploy to Shuttle.rs or similar Rust hosting

## 🛠️ Environment Variables

Create `.env` file in project root:

```env
# Database (development uses SQLite, production uses PostgreSQL)
DATABASE_URL=sqlite:fishing.db

# Server
BACKEND_HOST=127.0.0.1
BACKEND_PORT=8080
FRONTEND_PORT=3001

# Logging
RUST_LOG=info

# Optional: External APIs
# OPEN_METEO_API_KEY=...
```

## 📊 Project Status

- ✅ **Phase 1**: MVP with map, weather, user location
  - Interactive Leaflet map
  - Real Open-Meteo weather data
  - Browser geolocation with wind direction
  - Forecast probability calculation
  
- 🔄 **Phase 2**: Backend integration (current)
  - SQLite/PostgreSQL support
  - REST API endpoints
  - Catch logging functionality
  - User authentication

- 📋 **Phase 3**: Advanced features
  - ML-based bite prediction
  - Historical data analysis
  - Offline mode
  - Mobile app (Capacitor)
  - Desktop app (Tauri)

## 🐛 Troubleshooting

**Port already in use:**
```bash
# Windows
netstat -ano | findstr ":3001"
taskkill /PID <PID> /F

# macOS/Linux
lsof -i :3001
kill -9 <PID>
```

**Trunk build error:**
```bash
rm -rf .trunk target
trunk build
```

**Backend won't connect:**
```bash
# Check if running
netstat -ano | findstr ":8080"

# Check DATABASE_URL
echo $DATABASE_URL

# Run with debug logging
RUST_LOG=debug cargo run
```

**CORS/Connection errors:**
- Backend has CORS enabled for all origins in development
- Check that both servers are running on correct ports
- Try accessing http://localhost:8080/api/v1/health in browser

### Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/fishing-forecast.git
   cd fishing-forecast
   ```

2. **Setup database**
   ```bash
   createdb fishing_forecast
   psql fishing_forecast -c "CREATE EXTENSION postgis;"
   ```

3. **Configure environment**
   ```bash
   cd crates/backend
   cp .env.example .env
   # Edit .env with your DATABASE_URL
   ```

4. **Run migrations**
   ```bash
   cd crates/backend
   sqlx migrate run
   ```

5. **Start backend**
   ```bash
   cargo run -p fishing-backend
   ```

6. **Start frontend** (in new terminal)
   ```bash
   cd crates/frontend
   trunk serve
   ```

7. **Open browser**
   ```
   http://localhost:8080
   ```

## 📖 Documentation

- [Database Schema](docs/DATABASE_SCHEMA.md)
- [API Documentation](docs/API.md) - Coming soon
- [Deployment Guide](docs/DEPLOYMENT.md) - Coming soon
- [Contributing Guide](CONTRIBUTING.md) - Coming soon

## 🧪 Development

### Run tests
```bash
cargo test --workspace
```

### Linting
```bash
cargo clippy -- -D warnings
```

### Formatting
```bash
cargo fmt --all
```

### Build for production
```bash
# Backend
cargo build --release -p fishing-backend

# Frontend
cd crates/frontend && trunk build --release
```

## 🌍 Supported Regions

| Country | Fish Species | Regulations | Status |
|---------|-------------|-------------|--------|
| 🇺🇦 Ukraine | 100% | 100% | ✅ Complete |
| 🇵🇱 Poland | 70% | 70% | 🚧 In Progress |
| 🇩🇪 Germany | 0% | 0% | 📋 Planned |

## 📊 Project Status

**Current Phase**: Phase 1 - Week 1 (Backend Foundation)  
**Target MVP Launch**: Week 16  
**Progress**: 0% (Just started! 🚀)

## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) first.

## 📝 License

This project is dual-licensed under MIT OR Apache-2.0. See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) for details.

## 🙏 Acknowledgments

- [Open-Meteo](https://open-meteo.com/) - Weather data API
- [OpenStreetMap](https://www.openstreetmap.org/) - Water bodies and geocoding
- [Dioxus](https://dioxuslabs.com/) - Rust frontend framework
- [Axum](https://github.com/tokio-rs/axum) - Web framework

---

Made with ❤️ and 🦀 Rust
