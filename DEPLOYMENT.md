# 🚀 Deployment Instructions

## GitHub Setup

### 1. Create GitHub Repository

```bash
# Option A: Using GitHub CLI (if installed)
gh repo create fishing-forecast --public --source=. --push

# Option B: Manually on GitHub.com
# 1. Go to https://github.com/new
# 2. Create repo "fishing-forecast" (public)
# 3. Run:
git remote add origin https://github.com/YOUR_USERNAME/fishing-forecast.git
git branch -M main
git push -u origin main
```

---

## Neon PostgreSQL Setup

1. Go to [Neon.tech](https://neon.tech)
2. Sign up (free tier)
3. Create new project: "fishing-forecast"
4. Copy connection string (format: `postgres://user:pass@ep-xxx.us-east-1.aws.neon.tech/fishing-forecast`)

---

## GitHub Secrets Setup

### Shuttle.rs API Key
1. Go to [Shuttle.rs](https://shuttle.rs) and sign up
2. Get your API key from dashboard
3. Add to GitHub: **Settings → Secrets → Actions → New repository secret**
   - Name: `SHUTTLE_API_KEY`
   - Value: your_api_key

### Cloudflare Setup
1. Sign up at [Cloudflare](https://cloudflare.com)
2. Go to **Workers & Pages → Pages → Connect to Git**
3. Authorize GitHub access
4. Select "fishing-forecast" repo
5. Copy Account ID from **Workers & Pages → Overview**
6. Create API token: **My Profile → API Tokens → Create Custom Token**
   - Permissions: Pages: Read, Write
7. Add secrets to GitHub:
   - `CLOUDFLARE_API_TOKEN`: your_token
   - `CLOUDFLARE_ACCOUNT_ID`: your_account_id

---

## Trigger Deployment

1. Push to main branch:
```bash
git push origin main
```

2. GitHub Actions will automatically:
   - Build & deploy backend to Shuttle.rs
   - Build & deploy frontend to Cloudflare Pages

---

## URLs After Deployment

| Service | URL |
|---------|-----|
| Frontend | https://fishing-forecast.pages.dev |
| Backend API | https://fishing-forecast-api.shuttleapp.rs |
| Health Check | https://fishing-forecast-api.shuttleapp.rs/api/v1/health |

---

## Troubleshooting

### Backend deploy fails
- Check `DATABASE_URL` is set in Shuttle secrets
- Ensure Neon connection string is valid

### Frontend deploy fails
- Check Cloudflare credentials in secrets
- Ensure `manifest.json` and `sw.js` exist in `dist/`

### CORS errors
- Update CORS settings in `main.rs` to allow your frontend domain
