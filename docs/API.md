# Fishing Forecast API Documentation

## Base URL

Development: `http://localhost:8080/api/v1`
Production: `https://api.fishing-forecast.com/api/v1`

## Authentication

All endpoints (except auth and public ones) require JWT token in Authorization header:

```
Authorization: Bearer <your_jwt_token>
```

## Endpoints

### Health Check

**GET** `/health`

Returns API health status.

**Response:**
```json
{
  "status": "healthy",
  "version": "1.0.0",
  "timestamp": "2026-02-07T20:00:00Z"
}
```

---

### Authentication

#### Register User

**POST** `/auth/register`

**Request Body:**
```json
{
  "email": "user@example.com",
  "password": "secure_password",
  "country_code": "UA",
  "language": "uk"
}
```

**Response (201 Created):**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "uuid-string",
    "email": "user@example.com",
    "country_code": "UA",
    "language": "uk",
    "created_at": "2026-02-07T20:00:00Z"
  }
}
```

**Error (400 Bad Request):**
```json
{
  "error": "User with this email already exists"
}
```

#### Login User

**POST** `/auth/login`

**Request Body:**
```json
{
  "email": "user@example.com",
  "password": "secure_password"
}
```

**Response (200 OK):**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "uuid-string",
    "email": "user@example.com",
    "country_code": "UA",
 "uk",
       "language": "created_at": "2026-02-07T20:00:00Z"
  }
}
```

**Error (401 Unauthorized):**
```json
{
  "error": "Invalid email or password"
}
```

#### Get Current User

**GET** `/auth/me`

Requires authentication.

**Response (200 OK):**
```json
{
  "id": "uuid-string",
  "email": "user@example.com",
  "country_code": "UA",
  "language": "uk",
  "created_at": "2026-02-07T20:00:00Z"
}
```

---

### Forecast

#### Get Bite Forecast

**GET** `/forecast`

**Query Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| lat | float | Yes | Latitude (-90 to 90) |
| lon | float | Yes | Longitude (-180 to 180) |
| fish | string | No | Fish species ID (optional) |

**Response (200 OK):**
```json
{
  "probability": 0.75,
  "rating": "good",
  "factors": {
    "weather": 0.8,
    "moon": 0.9,
    "time": 0.7,
    "wind": 0.6
  },
  "location": {
    "lat": 50.45,
    "lon": 30.52
  },
  "timestamp": "2026-02-07T20:00:00Z"
}
```

**Rating Values:**
- `excellent` - 0.8-1.0
- `good` - 0.6-0.8
- `moderate` - 0.4-0.6
- `poor` - 0.2-0.4
- `bad` - 0.0-0.2

#### Get Detailed Forecast

**GET** `/forecast/detailed`

**Query Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| lat | float | Yes | Latitude |
| lon | float | Yes | Longitude |
| fish | string | No | Fish species ID |

**Response (200 OK):**
```json
{
  "probability": 0.75,
  "rating": "good",
  "factors": [
    {
      "name": "weather",
      "score": 0.8,
      "weight": 0.3,
      "description": "Temperature 18°C is optimal"
    },
    {
      "name": "moon",
      "score": 0.9,
      "weight": 0.2,
      "description": "Moon phase 0.75 is favorable"
    },
    {
      "name": "time",
      "score": 0.7,
      "weight": 0.2,
      "description": "Morning hours are good"
    },
    {
      "name": "wind",
      "score": 0.6,
      "weight": 0.3,
      "description": "Light wind 4 m/s"
    }
  ],
  "location": {
    "lat": 50.45,
    "lon": 30.52
  },
  "best_times": ["06:00-09:00", "18:00-21:00"],
  "timestamp": "2026-02-07T20:00:00Z"
}
```

#### Get Feature Importance

**GET** `/forecast/importance`

Returns ML model feature importance weights.

**Response (200 OK):**
```json
{
  "features": [
    {"name": "temperature", "importance": 0.25},
    {"name": "pressure", "importance": 0.20},
    {"name": "wind_speed", "importance": 0.18},
    {"name": "moon_phase", "importance": 0.15},
    {"name": "time_of_day", "importance": 0.12},
    {"name": "day_of_year", "importance": 0.10}
  ]
}
```

---

### Fish Species

#### Get Fish Species

**GET** `/fish`

**Query Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| country | string | Country code (UA, PL, DE) |
| language | string | Language code (uk, en, pl, de) |

**Response (200 OK):**
```json
{
  "species": [
    {
      "id": "pike",
      "name_uk": "Щука",
      "name_en": "Pike",
      "scientific_name": "Esox lucius",
      "best_season": "all_year",
      "preferred_bait": "spoon",
      "min_temp": 5.0,
      "max_temp": 22.0
    },
    {
      "id": "carp",
      "name_uk": "Короп",
      "name_en": "Carp",
      "scientific_name": "Cyprinus carpio",
      "best_season": "summer",
      "preferred_bait": "boilie",
      "min_temp": 18.0,
      "max_temp": 30.0
    }
  ]
}
```

---

### Region

#### Detect Region

**GET** `/region/detect`

**Query Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| lat | float | Yes | Latitude |
| lon | float | Yes | Longitude |

**Response (200 OK):**
```json
{
  "region": "UA-32",
  "name": "Київська область",
  "country_code": "UA",
  "regulations_applied": true
}
```

---

### Regulations

#### Get Regulations

**GET** `/regulations`

**Query Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| country_code | string | Country code (UA, PL, DE) |
| region | string | Region code (optional) |
| fish_species | string | Fish species ID (optional) |

**Response (200 OK):**
```json
{
  "regulations": [
    {
      "id": "uuid",
      "country_code": "UA",
      "region": "UA-32",
      "fish_species": "pike",
      "min_size_cm": 40,
      "max_size_cm": 70,
      "daily_limit": 3,
      "season_start": null,
      "season_end": null,
      "protected": false,
      "license_required": true,
      "license_url": "https://example.com/license",
      "notes": "Заборонено ловити в нерестовий період"
    }
  ]
}
```

#### Validate Catch Legality

**POST** `/regulations/validate`

**Request Body:**
```json
{
  "country_code": "UA",
  "region": "UA-32",
  "fish_species": "pike",
  "weight_kg": 2.5,
  "length_cm": 45,
  "caught_at": "2026-02-07T20:00:00Z"
}
```

**Response (200 OK):**
```json
{
  "valid": true,
  "message": "Улов відповідає нормам"
}
```

**Response (400 Bad Request):**
```json
{
  "valid": false,
  "message": "Рибка занадто маленька. Мінімальний розмір: 40 см",
  "details": {
    "min_size_cm": 40,
    "caught_size_cm": 35
  }
}
```

---

### Catches

#### Save Catch Record

**POST** `/catches`

Requires authentication.

**Request Body:**
```json
{
  "location_lat": 50.45,
  "location_lon": 30.52,
  "caught_at": "2026-02-07T20:00:00Z",
  "fish_species": "pike",
  "weight_kg": 2.5,
  "length_cm": 45,
  "bait_used": "spoon",
  "weather_temp": 18.5,
  "weather_pressure": 1015.0,
  "moon_phase": 0.75,
  "notes": "Відмінний клювання біля затоки"
}
```

**Response (201 Created):**
```json
{
  "id": "uuid-string",
  "message": "Улов збережено"
}
```

#### Get User Catches

**GET** `/catches`

Requires authentication.

**Query Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| limit | int | Max records (default: 50) |
| offset | int | Pagination offset |

**Response (200 OK):**
```json
{
  "catches": [
    {
      "id": "uuid-string",
      "location_lat": 50.45,
      "location_lon": 30.52,
      "caught_at": "2026-02-07T20:00:00Z",
      "fish_species": "pike",
      "weight_kg": 2.5,
      "length_cm": 45,
      "bait_used": "spoon",
      "notes": "Відмінний клювання"
    }
  ],
  "total": 42,
  "limit": 50,
  "offset": 0
}
```

#### Get Nearby Catches

**GET** `/catches/nearby`

**Query Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| lat | float | Yes | Latitude |
| lon | float | Yes | Longitude |
| radius_km | float | Yes | Search radius in km |

**Response (200 OK):**
```json
{
  "catches": [
    {
      "id": "uuid-string",
      "location_lat": 50.45,
      "location_lon": 30.52,
      "fish_species": "pike",
      "caught_at": "2026-02-07T20:00:00Z"
    }
  ],
  "count": 5,
  "center": {
    "lat": 50.45,
    "lon": 30.52
  },
  "radius_km": 10
}
```

---

### Water Bodies

#### Get Water Bodies

**GET** `/water-bodies`

**Query Parameters:**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| lat | float | Yes | Latitude |
| lon | float | Yes | Longitude |
| radius_km | float | Yes | Search radius in km |
| country | string | No | Country code filter |

**Response (200 OK):**
```json
{
  "water_bodies": [
    {
      "id": "uuid-string",
      "name": "Київське водосховище",
      "description": "Велике водосховище на Дніпрі",
      "location_lat": 50.6,
      "location_lon": 30.5,
      "water_type": "reservoir",
      "country_code": "UA"
    }
  ],
  "count": 3,
  "center": {
    "lat": 50.45,
    "lon": 30.52
  },
  "radius_km": 20
}
```

---

## Error Codes

| Code | Description |
|------|-------------|
| 200 | Success |
| 201 | Created |
| 400 | Bad Request |
| 401 | Unauthorized |
| 403 | Forbidden |
| 404 | Not Found |
| 500 | Internal Server Error |
| 503 | Service Unavailable |

## Rate Limiting

- 100 requests per minute for authenticated users
- 20 requests per minute for unauthenticated users

## Versioning

API versioning is handled via URL path: `/api/v1/`

## Pagination

For endpoints returning lists, pagination is supported via `limit` and `offset` query parameters.

```bash
GET /catches?limit=10&offset=20
```

## Offline Support

When offline, cached API responses are returned for:
- Forecast data (cached for 1 hour)
- Fish species list (cached for 24 hours)
- Regulations (cached for 24 hours)

Cached data includes `cached_at` timestamp.
