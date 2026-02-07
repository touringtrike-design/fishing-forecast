### Stack
```
Frontend: React/Vue/Angular + Capacitor
Backend: Rust (Axum)
Mobile: Capacitor (компілює web → native)
Database: PostgreSQL + PostGIS
```

### Чому Capacitor:

- ✅ Web технології → iOS/Android/Desktop
- ✅ Один код для всіх платформ
- ✅ Легше за Flutter (якщо знаєте web)
- ✅ Доступ до нативних API

**Capacitor структура:**
```
fishing-app/
├── backend/             # Rust API server
├── web/                 # React/Vue app
│   ├── src/
│   └── package.json
└── capacitor.config.ts  # Capacitor config