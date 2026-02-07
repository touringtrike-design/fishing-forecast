## 🏆 ТОП-5 Безкоштовних варіантів

### 1. **Fly.io** ⭐ НАЙКРАЩЕ для Rust

**Безкоштовний план:**

- ✅ 3 shared-cpu VMs (256MB RAM кожна)
- ✅ 3GB persistent storage
- ✅ 160GB outbound transfer/місяць
- ✅ Автоматичні SSL сертифікати
- ✅ Глобальна мережа (edge locations)

**Чому ідеально для Rust:**

- ✅ Rust компілюється в маленький бінарник (~10MB)
- ✅ 256MB RAM **достатньо** для Rust (Node.js потребує 512MB+)
- ✅ Підтримує Dockerfile з коробки
- ✅ Postgres addon (також безкоштовний tier)

**Як задеплоїти:**

bash

```bash
# 1. Встановити flyctl
curl -L https://fly.io/install.sh | sh

# 2. Залогінитись
fly auth login

# 3. У папці вашого Rust проекту
fly launch

# 4. Deploy
fly deploy
```

**Dockerfile для Rust:**

dockerfile

```dockerfile
# Fly.io Dockerfile
FROM rust:1.75 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libpq5 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/fishing-api /app/fishing-api
EXPOSE 8080
CMD ["/app/fishing-api"]
```

**fly.toml:**

toml

```toml
app = "fishing-forecast-api"

[build]

[env]
  PORT = "8080"

[[services]]
  http_checks = []
  internal_port = 8080
  protocol = "tcp"

  [[services.ports]]
    port = 80
    handlers = ["http"]

  [[services.ports]]
    port = 443
    handlers = ["tls", "http"]

[mounts]
  source = "fishing_data"
  destination = "/data"
```

**Обмеження:**

- ⚠️ VMs засинають після 5 хв неактивності (cold start ~2-3 сек)
- ⚠️ 3GB storage (достатньо для старту)

---

### 2. **Railway.app** ⭐⭐ Дуже простий

**Безкоштовний план:**

- ✅ $5 кредитів/місяць (вистачає на ~500 годин роботи)
- ✅ 512MB RAM
- ✅ 1GB storage
- ✅ PostgreSQL included
- ✅ Автодеплой з GitHub

**Чому класно:**

- ✅ **Найпростіший setup** (3 кліки)
- ✅ Автоматично детектує Rust проект
- ✅ Вбудований Postgres
- ✅ Гарний UI/dashboard

**Як задеплоїти:**

bash

```bash
# 1. Підключити GitHub repo
# 2. Railway автоматично:
#    - Виявить що це Rust
#    - Створить Dockerfile
#    - Задеплоїть
# 3. Додати Postgres через UI

# Або через CLI:
npm install -g @railway/cli
railway login
railway init
railway up
```

**Обмеження:**

- ⚠️ $5/місяць вистачає на ~20 днів постійної роботи
- ⚠️ Після цього треба платити або app спить

---

### 3. **Render.com** ⭐⭐ Добрий баланс

**Безкоштовний план:**

- ✅ 750 годин/місяць (достатньо для 1 сервісу 24/7)
- ✅ 512MB RAM
- ✅ Автоматичний SSL
- ✅ PostgreSQL: 90 днів retention, 1GB storage

**Переваги:**

- ✅ Не засинає (якщо в межах 750 годин)
- ✅ Легкий deploy з Git
- ✅ Підтримка Rust з коробки

**Налаштування:**

**render.yaml:**

yaml

```yaml
services:
  - type: web
    name: fishing-api
    env: rust
    buildCommand: cargo build --release
    startCommand: ./target/release/fishing-api
    envVars:
      - key: DATABASE_URL
        fromDatabase:
          name: fishing-db
          property: connectionString
      - key: PORT
        value: 8080

databases:
  - name: fishing-db
    databaseName: fishing
    user: fishing_user
    plan: free
```

**Обмеження:**

- ⚠️ Безкоштовний Postgres тільки 90 днів
- ⚠️ Після 15 хв неактивності засинає (cold start ~30 сек)

---

### 4. **Shuttle.rs** ⭐⭐⭐ Спеціально для Rust!

**Безкоштовний план:**

- ✅ Unlimited deployments
- ✅ 2GB RAM (!)
- ✅ Shared Postgres included
- ✅ Made for Rust

**Чому унікальний:**

- ✅ Створений **СПЕЦІАЛЬНО для Rust**
- ✅ Макроси для простого deploy
- ✅ Вбудований Postgres, Redis
- ✅ Найпростіший Rust deploy

**Приклад коду:**

rust

```rust
// Cargo.toml
[dependencies]
shuttle-runtime = "0.38.0"
shuttle-axum = "0.38.0"
shuttle-shared-db = { version = "0.38.0", features = ["postgres"] }
axum = "0.7"
sqlx = "0.7"

// src/main.rs
use axum::{routing::get, Router};
use shuttle_axum::ShuttleAxum;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: sqlx::PgPool,
) -> ShuttleAxum {
    // Міграції автоматично
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    
    let router = Router::new()
        .route("/", get(|| async { "Fishing Forecast API" }))
        .route("/api/forecast", get(get_forecast))
        .with_state(pool);
    
    Ok(router.into())
}

async fn get_forecast() -> &'static str {
    "Forecast endpoint"
}
```

**Deploy:**

bash

```bash
# Встановити Shuttle CLI
cargo install cargo-shuttle

# Ініціалізувати
cargo shuttle init

# Deploy
cargo shuttle deploy
```

**Це НАЙЛЕГШИЙ спосіб задеплоїти Rust!** 🚀

**Обмеження:**

- ⚠️ Молодий проект (може бути нестабільним)
- ⚠️ Менше контролю над інфраструктурою

---

### 5. **Koyeb** ⭐ Ще один варіант

**Безкоштовний план:**

- ✅ 1 web service
- ✅ 512MB RAM
- ✅ 2.5GB storage
- ✅ 100GB bandwidth

**Переваги:**

- ✅ Не засинає
- ✅ Edge network
- ✅ Docker або Git deploy

---

## 🗄️ Безкоштовні бази даних

### 1. **Neon.tech** ⭐ НАЙКРАЩЕ для Postgres

**Безкоштовний план:**

- ✅ 0.5GB storage
- ✅ Autoscaling (до 0 коли не використовується)
- ✅ Необмежені databases
- ✅ Branching (для тестування)
- ✅ **PostGIS supported!** ⭐

bash

```bash
# Connection string
postgresql://user:pass@ep-xyz.eu-central-1.aws.neon.tech/fishing?sslmode=require
```

**Чому ідеально:**

- ✅ Serverless Postgres
- ✅ Підтримує PostGIS (для геоданих!)
- ✅ Не треба управляти сервером
- ✅ Автоматично скейлиться до 0

---

### 2. **Supabase** ⭐⭐ Все в одному

**Безкоштовний план:**

- ✅ 500MB database
- ✅ Postgres + PostGIS
- ✅ Автентифікація
- ✅ Storage (1GB)
- ✅ Real-time subscriptions
- ✅ Edge Functions

**Переваги:**

- ✅ Backend-as-a-Service
- ✅ Можна не писати свій backend!
- ✅ REST API автогенерується

**З Rust:**

rust

```rust
// Можна використовувати Supabase тільки як БД
let pool = PgPool::connect(
    "postgresql://postgres:[password]@db.xxx.supabase.co:5432/postgres"
).await?;
```

**Або повністю client-side з Flutter:**

dart

```dart
// Flutter → Supabase (без свого Rust backend!)
import 'package:supabase_flutter/supabase_flutter.dart';

await Supabase.initialize(
  url: 'https://xxx.supabase.co',
  anonKey: 'your-anon-key',
);

// Зберегти улов
await supabase.from('catches').insert({
  'location': 'POINT(30.5 50.4)',
  'fish': 'carp',
});
```

---

### 3. **ElephantSQL** ⭐ Простий Postgres

**Безкоштовний план:**

- ✅ 20MB storage (мало, але безкоштовно)
- ✅ Shared server
- ✅ PostGIS available

---

### 4. **Turso** (Libsql/SQLite) ⭐⭐ Цікава альтернатива

**Безкоштовний план:**

- ✅ 500 databases
- ✅ 1 billion rows read/місяць
- ✅ 25 million rows write/місяць
- ✅ Edge реплікація

**Чому цікаво:**

- ✅ SQLite-like (простіше за Postgres)
- ✅ Величезні ліміти
- ✅ Дуже швидкий

**Мінуси:**

- ❌ Немає PostGIS

---

## 🎯 Моя рекомендація для "Прогноз клювання"

### **Варіант A: Мінімальний backend** (рекомендую)

yaml

```yaml
Frontend: Flutter (client-side)
  - Погода: Open-Meteo API (direct)
  - Карти: OSM (direct)
  - AI: Local rules-based

Backend (тільки для синхронізації):
  - Supabase Free Tier
  - PostgreSQL + PostGIS
  - Authentication
  
Total cost: $0/місяць ✅
Обмеження: 500MB DB, 50K API requests/day
```

**Це покриє 90% функціоналу БЕЗ свого backend!**

---

### **Варіант B: Повний Rust backend**

yaml

```yaml
Application Server:
  - Shuttle.rs (безкоштовно)
  - Rust + Axum
  - 2GB RAM

Database:
  - Neon.tech (безкоштовно)
  - PostgreSQL + PostGIS
  - 0.5GB storage

Cache (опційно):
  - Upstash Redis (10K commands/day безкоштовно)

Total cost: $0/місяць ✅
Обмеження: 
  - 0.5GB DB
  - Sleep після неактивності
```

---

### **Варіант C: Максимальний uptime**

yaml

```yaml
Application:
  - Fly.io (3 VMs безкоштовно)
  - Rust + Axum
  - Load balanced

Database:
  - Supabase Postgres (500MB)

CDN:
  - Cloudflare (безкоштовно)

Total cost: $0/місяць ✅
Uptime: 99%+
```

---

## 📊 Порівняльна таблиця

|Сервіс|RAM|Storage|DB|Sleep?|Найкраще для|
|---|---|---|---|---|---|
|**Shuttle.rs**|2GB|-|✅ Postgres вбудований|Так (15хв)|Простий Rust deploy|
|**Fly.io**|256MB×3|3GB|Окремо|Так (5хв)|Production-ready apps|
|**Railway**|512MB|1GB|✅ PostgreSQL|Так ($5 кредит)|Найпростіший UI|
|**Render**|512MB|-|✅ 90 днів|Так (15хв)|Стабільний хостинг|
|**Supabase**|-|-|✅ 500MB + backend|Ні|BaaS без свого API|