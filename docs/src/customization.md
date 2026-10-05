# Customization & Theming

Arcadia allows administrators to personalize the visual branding, landing pages, stylesheets, and assets of their instance.

## Custom Landing Page

By default, visiting the root URL of an Arcadia instance displays the login view. If you wish to present a custom HTML landing page to unauthenticated visitors:

1. In `config.yml`, set:
   ```yaml
   frontend:
     enable_custom_front_page: true
   ```
2. Place your custom HTML file at `public/home/index.html` (inside the frontend directory or container).
3. When enabled, non-logged-in visitors landing on `/` are served this page, while authenticated users are directed to the home page (`HomeView.vue`).

## Unauthenticated Pages (Custom CSS & JS)

Pages accessible before signing in (`/login`, `/register`, `/apply`, `/reset-password`) cannot use the user-selectable CSS stylesheets configured in the database, because no user session exists. 

To apply custom branding to these pages, create two optional, git-ignored files:

- `frontend/public/custom_unauth.css`: Custom CSS rules loaded on unauthenticated pages.
- `frontend/public/custom_unauth.js`: Custom JavaScript executed on unauthenticated pages.

> [!NOTE]
> Creating or editing these files requires rebuilding the frontend (`docker compose build frontend`) and switching to the new image (`docker compose up -d frontend`) 

## Site Assets & Branding

You can replace the default placeholder graphics with your own site branding by replacing the following asset files:

| Asset | File Location | Purpose |
| :--- | :--- | :--- |
| **Site Logo** | `frontend/src/assets/logo.svg` | Main navbar logo (falls back to `logo.example.svg` if omitted) |
| **Favicon** | `frontend/public/favicon.ico` | Browser tab icon |
| **Default Avatar** | `frontend/public/default_user_avatar.png` | Fallback avatar for users who haven't uploaded one |
| **Bonus Points Icon**| `frontend/public/bonus_points_icon.png` | Currency icon displayed next to bonus point balances |

### Applying asset updates to Docker container

When building the Docker image, assets are inlined into the compiled bundle. After replacing any assets in `frontend/src/assets/` or `frontend/public/`, rebuild the frontend container:

```bash
docker compose build frontend
docker compose up -d frontend
```

## User Stylesheets (CSS Sheets)

Once users are logged in, Arcadia supports custom theme stylesheets. Users with the `create_css_sheet` and `edit_css_sheet` permissions can add and manage site-wide stylesheets through the Staff Dashboard:

- Each stylesheet is stored in the database (`css_sheets` table) with a unique name, CSS content, and an optional preview image.
- Users can select their preferred active theme in their account settings.
- The instance's default theme is defined in the site settings (`default_css_sheet_name`, defaults to `arcadia`).
