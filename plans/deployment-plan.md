# 🚀 DEPLOYMENT PLAN - Fishing Forecast

## Overview
Deploy the Fishing Forecast application to production using:
- **Backend**: Shuttle.rs (free tier: 2GB RAM)
- **Frontend**: Cloudflare Pages (free, unlimited bandwidth)
- **Database**: Neon PostgreSQL (free tier: 0.5GB)

---

## Step 1: Prepare for Deployment

### 1.1 Create Shuttle.toml for Backend

```toml
# crates/backend/Shuttle.toml
name = "fishing-forecast"
deployment-id = "fishing-forecast-api"
project-id = "your-project-id"

[secrets]
DATABASE_URL = { type = "pg", }
JWT_SECRET = { type = "string", }
```

### 1.2 Update Backend for Shuttle

```rust
// crates/backend/src/main.rs
use shuttle_runtime::Secret;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: Secret,
) -> Result<App, shuttle_runtime::Error> {
    let database_url = secrets
        .get("DATABASE_URL")
        .expect("DATABASE_URL not set");
    let jwt_secret = secrets
        .get("JWT_SECRET")
        .expect("JWT_SECRET not set");

    // ... existing code with secrets ...

    Ok(app)
}
```

### 1.3 Add Shuttle Dependencies

```toml
# crates/backend/Cargo.toml
[dependencies]
shuttle-runtime = "0.48"
shuttle-shared-db = { version = "0.48", features = ["postgres"] }
```

---

## Step 2: GitHub Actions Workflows

### 2.1 Backend Deploy Workflow

```yaml
# .github/workflows/deploy-backend.yml
name: Deploy Backend to Shuttle

on:
  push:
    branches: [main]
    paths:
      - 'crates/backend/**'

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Install Shuttle
        run: cargo install shuttle-cli

      - name: Deploy to Shuttle
        run: |
          cd crates/backend
          shuttle deploy --api-key ${{ secrets.SHUTTLE_API_KEY }}
        env:
          SHUTTLE_API_KEY: ${{ secrets.SHUTTLE_API_KEY }}
```

### 2.2 Frontend Deploy Workflow

```yaml
# .github/workflows/deploy-frontend.yml
name: Deploy Frontend to Cloudflare Pages

on:
  push:
    branches: [main]
    paths:
      - 'crates/frontend/**'

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Build WASM
        run: |
          cd crates/frontend
          cargo install trunk
          trunk build --release

      - name: Deploy to Cloudflare Pages
        uses: cloudflare/pages-action@v1
        with:
          apiToken: ${{ secrets.CLOUDFLARE_API_TOKEN }}
          accountId: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
          projectName: fishing-forecast
          directory: crates/frontend/dist
          # Optional: set env variables
          # wranglerVars: '{"API_URL":"https://api.fishing-forecast.com"}'
```

---

## Step 3: Database Setup (Neon.tech)

### 3.1 Create Neon Project
1. Go to [Neon.tech](https://neon.tech)
2. Create new project "fishing-forecast"
3. Copy connection string: `postgres://user:pass@ep-xxx.us-east-1.aws.neon.tech/fishing-forecast`

### 3.2 Run Migrations
```bash
# Connect to Neon DB
DATABASE_URL="postgres://..." sqlx database create
DATABASE_URL="..." sqlx migrate run
```

### 3.3 Add Connection Pooling (Recommended)
Neon requires connection pooling for serverless. Add PgBouncer:
```
Host: ep-xxx.us-east-1.aws.neon.tech:5433
Database: fishing-forecast
Pool mode: transaction
```

---

## Step 4: Environment Configuration

### 4.1 Backend Secrets (Shuttle Secrets)
```bash
# Set secrets via Shuttle CLI or GitHub Actions
SHUTTLE_API_KEY=your_api_key
DATABASE_URL=postgres://... (Neon connection string)
JWT_SECRET=your_super_secret_jwt_key
```

### 4.2 Frontend API URL
Update `crates/frontend/src/services/api_client.rs`:
```rust
const API_BASE_URL: &str = if cfg!(debug_assertions) {
    "http://127.0.0.1:8080"
} else {
    "https://fishing-forecast-api.shuttleapp.rs"
};
```

---

## Step 5: Deploy Steps

### 5.1 GitHub Setup
```bash
# Initialize git if not done
git init
git add .
git commit -m "Initial commit: MVP ready for deployment"

# Create GitHub repo
gh repo create fishing-forecast --public --source=. --push

# Add secrets to GitHub (Settings → Secrets)
# SHUTTLE_API_KEY
# CLOUDFLARE_API_TOKEN
# CLOUDFLARE_ACCOUNT_ID
```

### 5.2 Deploy Backend
```bash
# Manual deploy (first time)
cd crates/backend
cargo shuttle deploy

# Or via GitHub Actions (automatic on push to main)
```

### 5.3 Deploy Frontend
```bash
# Build and upload
cd crates/frontend
trunk build --release

# Upload dist/ to Cloudflare Pages
# Or use GitHub Actions (automatic on push to main)
```

---

## URLs After Deployment

| Component | URL |
|-----------|-----|
| Frontend | https://fishing-forecast.pages.dev |
| Backend | https://fishing-forecast-api.shuttleapp.rs |
| API Docs | https://fishing-forecast-api.shuttleapp.rs/docs |
| Health | https://fishing-forecast-api.shuttleapp.rs/health |

---

## Rollback Strategy

### Backend Rollback
```bash
cd crates/backend
cargo shuttle deploy --rollback
```

### Frontend Rollback
Cloudflare Pages keeps deployment history. Revert via dashboard.

---

## Monitoring

### Health Checks
- Backend: `GET /health`
- Frontend: Check Cloudflare analytics

### Logs
- Backend: `cargo shuttle logs`
- Frontend: Cloudflare Pages logs

---

## Estimated Cost
- **Backend (Shuttle)**: $0/month (2GB RAM free)
- **Frontend (Cloudflare)**: $0/ month (unlimited bandwidth)
- **Database (Neon)**: $0/month (0.5GB free)

**Total: $0/month** 🎉
