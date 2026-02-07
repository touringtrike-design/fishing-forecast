Stack
Frontend: Flutter
Backend: Supabase (BaaS)
Карти: MapLibre GL + OSM
AI/ML: Google Cloud AI Platform (готові API)
Хостинг: Supabase (все в одному)
### Чому це найшвидше:

**Flutter** (замість React Native):

- ✅ Менше нативних багів - один рендер для iOS/Android
- ✅ Hot reload працює стабільніше
- ✅ Вбудовані компоненти Material/Cupertino
- ✅ Краща продуктивність "з коробки"
- ✅ Один код = iOS + Android + Web

**Supabase** (замість власного бекенду):

- ✅ PostgreSQL + PostGIS вже є
- ✅ Автентифікація готова (email, Google, Apple)
- ✅ Real-time підписки з коробки
- ✅ Storage для фото уловів
- ✅ Edge Functions для логіки
- ✅ Auto-generated REST API
- ⚡ **Не треба писати бекенд!**
Технічний стек:
Mobile: Flutter 3.x
Backend: Supabase
  - PostgreSQL 15 + PostGIS
  - Edge Functions (Deno)
  - Storage (S3-сумісне)
  - Auth (готова)
Карти: flutter_map + MapLibre
Погода: OpenWeatherMap API
AI: Supabase Edge Functions + OpenAI API
State: Riverpod (легкий state management)