# Reseed Request Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Let a permitted user ask every past seeder/snatcher of a long-dead torrent to seed again, via a new auto-clearing in-app notification type.

**Architecture:** A new `reseed_request` notification type follows Arcadia's existing per-type pattern (own table, `NotificationEvent` variant, repo SQL, SSE event, Vue component). A permission-gated `POST` endpoint validates eligibility (zero seeders, dead ≥ a configurable number of hours derived from `MAX(torrent_activities.last_seen_seeding_at)`) and notifies past seeders ∪ snatchers. A new `tokio-cron-scheduler` job hard-deletes the notifications once a torrent is healthy (`seeders >= 1 AND leechers = 0`). The title-group torrent view gains a `last_seeded_at` field and a reseed button.

**Tech Stack:** Rust (actix-web 4, sqlx 0.9, PostgreSQL, tokio-cron-scheduler), Vue 3 + TypeScript + PrimeVue + Pinia, OpenAPI-generated API client.

**Spec:** `docs/superpowers/specs/2026-10-02-reseed-request-design.md`

## Global Constraints

- **Single migration file only.** All schema changes go in `backend/storage/migrations/20250312215600_initdb.sql`. Do NOT create new migration files (per `backend/CLAUDE.md`).
- **Do NOT run `cargo sqlx prepare` or a release build.** After editing schema/SQL, the human author rebuilds the dev DB and regenerates the `.sqlx` offline cache. `cargo test` requires a running dev DB (`DATABASE_URL` set, via `backend/storage/scripts/init_db.sh`). Agents verify with `cargo clippy` always; run `cargo test` when a dev DB is available, otherwise state that tests are pending the author's DB/cache rebuild.
- **Never use raw axios** on the frontend — always the generated `api-schema` client. The client regenerates from the OpenAPI spec; do not hand-edit files under `frontend/src/services/api-schema/`.
- **Permission literal naming:** Postgres enum literal `'request_reseed'` ↔ Rust variant `UserPermission::RequestReseed` (snake_case via `#[sqlx(rename_all="snake_case")]`).
- **Setting name:** `reseed_requestable_after_hours` (public setting), `INT NOT NULL DEFAULT 72`.
- **Recipients rule (verbatim):** users in `torrent_activities` for the torrent where `first_seen_seeding_at IS NOT NULL OR completed_at IS NOT NULL`, excluding the requester.
- **Auto-clear rule (verbatim):** hard-DELETE reseed notifications for torrents where `seeders >= 1 AND leechers = 0`.
- **SSE event string:** `"reseed_request"`.

## Review Focus

- **Torrent never had a seeder** (`MAX(last_seen_seeding_at) IS NULL`): the trigger must reject, not notify zero users silently as if successful. Covered in Task 1, Step "trigger rejects ineligible".
- **Requester is themselves a past seeder/snatcher:** they must be excluded from recipients (`user_id <> $requester`). Covered in Task 1 recipient-insert test.
- **Re-trigger while an unread request already exists:** must not create duplicate rows (unread `WHERE NOT EXISTS` guard). Covered in Task 1 dedup test.
- **Torrent healthy but leechers still present** (`seeders >= 1 AND leechers > 0`): auto-clear must NOT delete yet. Covered in Task 1 auto-clear test.
- **Permitted user but torrent still has seeders:** reject with a distinct eligibility error, not a permission error. Covered in Task 1 trigger test.

---

## Task 1: Backend — schema, permission, setting, trigger, notification, auto-clear, display

**Files:**
- Modify: `backend/storage/migrations/20250312215600_initdb.sql` (permission enum literal; `arcadia_settings` column; new `notifications_reseed_requests` table + indexes)
- Modify: `backend/storage/src/models/user.rs:68-177` (`UserPermission::RequestReseed`)
- Modify: `backend/storage/src/models/arcadia_settings.rs:81-137` and `:158-178` (`reseed_requestable_after_hours` on `ArcadiaSettings` + `PublicArcadiaSettings`)
- Modify: `backend/storage/src/repositories/arcadia_settings_repository.rs` (SELECT 17-64; UPDATE/RETURNING/binds 81-221)
- Modify: `backend/api/src/handlers/arcadia_settings/update_arcadia_settings.rs:51-81` (validation)
- Modify: `backend/storage/src/models/notification.rs` (enum variant + `user_ids()`/`event_type()`; `NotificationReseedRequest`; `Notifications` + `NotificationCounts` fields)
- Modify: `backend/storage/src/repositories/notification_repository.rs` (`notify_users_reseed_request`; SELECT in `find_all_notifications`; count in `find_notification_counts`)
- Create: `backend/api/src/handlers/torrents/request_reseed.rs`
- Modify: `backend/api/src/handlers/torrents/mod.rs` (route), `backend/api/src/api_doc.rs` (path), `backend/api/src/middlewares/api_key_scopes.rs` (scope)
- Modify: `backend/storage/src/models/torrent.rs` (`TorrentHierarchy` ~613 — add `last_seeded_at`)
- Modify: `backend/storage/src/repositories/title_group_repository.rs:164-196` (SELECT) and `:455-533` (build loop)
- Create: `backend/periodic-tasks/src/periodic_tasks/reseed_requests.rs`
- Modify: `backend/periodic-tasks/src/periodic_tasks/mod.rs` (mod entry), `backend/periodic-tasks/src/config.rs:10-31` (config field), `backend/periodic-tasks/src/periodic_tasks/scheduler.rs` (register job), `config.example.yml`
- Create: `backend/api/tests/test_reseed_request.rs`
- Reference: `backend/api/tests/fixtures/with_test_users_with_permissions.sql`

**Interfaces:**
- Produces (consumed by Task 2 via OpenAPI):
  - Endpoint `POST /api/torrents/{id}/reseed-request` → `200 OK` empty body on success; `403` InsufficientPermissions; `409`/domain error when ineligible.
  - `PublicArcadiaSettings.reseed_requestable_after_hours: i32`.
  - `TorrentHierarchy.last_seeded_at: Option<DateTime<Utc>>` (present only when `seeders == 0`).
  - `NotificationCounts.reseed_requests: i64` and `Notifications.reseed_requests: Vec<NotificationReseedRequest>`; SSE event string `"reseed_request"`.
- Consumes (from existing code): `arc.pool.require_permission`, `arc.notification_sender` (`broadcast::Sender<NotificationEvent>`), `arc.settings` (`Mutex<ArcadiaSettings>`).

### Schema

- [ ] **Step 1: Add the permission enum literal**

In `backend/storage/migrations/20250312215600_initdb.sql`, inside the `CREATE TYPE user_permissions_enum AS ENUM (...)` block, add a new literal:

```sql
    'request_reseed',
```

- [ ] **Step 2: Add the settings column**

In the `CREATE TABLE arcadia_settings (...)` block (lines ~290-336), add, next to the other `INT NOT NULL DEFAULT` numeric settings (e.g. after `min_amount_tags_title_group`):

```sql
    reseed_requestable_after_hours INT NOT NULL DEFAULT 72,
```

(The single-row seed `INSERT` at lines 337-338 need not change — the column has a default.)

- [ ] **Step 3: Add the notification table**

Add after the other `notifications_*` tables:

```sql
CREATE TABLE notifications_reseed_requests (
    id           BIGSERIAL PRIMARY KEY,
    torrent_id   INT NOT NULL REFERENCES torrents(id) ON DELETE CASCADE,
    user_id      INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    requested_by INT REFERENCES users(id) ON DELETE SET NULL,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    read_status  BOOLEAN NOT NULL DEFAULT FALSE
);
CREATE INDEX idx_notifications_reseed_requests_user ON notifications_reseed_requests(user_id, read_status);
CREATE INDEX idx_notifications_reseed_requests_torrent ON notifications_reseed_requests(torrent_id);
```

- [ ] **Step 4: Rebuild DB + sqlx cache (author action — note it)**

The schema file changed. State in the commit/PR notes: author must rerun `backend/storage/scripts/init_db.sh` (or `sqlx migrate run` against a fresh DB) and `cargo sqlx prepare` before offline compilation/tests pass. Do not run `cargo sqlx prepare` yourself.

- [ ] **Step 5: Commit**

```bash
git add backend/storage/migrations/20250312215600_initdb.sql
git commit -m "feat(db): add reseed request permission, setting, and notification table"
```

### Permission + Setting plumbing

- [ ] **Step 6: Add the Rust permission variant**

In `backend/storage/src/models/user.rs`, add to the `UserPermission` enum (follow the existing variant style, e.g. near `DeleteTorrent`):

```rust
    RequestReseed,
```

- [ ] **Step 7: Add the setting field to both structs**

In `backend/storage/src/models/arcadia_settings.rs`:
- Add to `ArcadiaSettings` (struct at lines 81-137), mirroring an existing `i32` field like `min_amount_tags_title_group`:

```rust
    pub reseed_requestable_after_hours: i32,
```

- Add the identical field to `PublicArcadiaSettings` (lines 158-178), and include it wherever `PublicArcadiaSettings` is constructed from `ArcadiaSettings` (follow how `min_amount_tags_title_group` or another numeric public field is copied there).

- [ ] **Step 8: Add the column to the three settings query sites**

In `backend/storage/src/repositories/arcadia_settings_repository.rs`:
- Add `reseed_requestable_after_hours` to the `SELECT` column list in `get_arcadia_settings` (lines 17-64).
- Add it to the `UPDATE ... SET` list, the `RETURNING` list, and the bind arguments in `update_arcadia_settings` (lines 81-221). Append it as the next `$N` (currently ends at `$45` → add as `$46`) so existing numbering is untouched, and add the matching `.bind(settings.reseed_requestable_after_hours)` in the same relative position.

- [ ] **Step 9: Add validation**

In `backend/api/src/handlers/arcadia_settings/update_arcadia_settings.rs`, in the numeric-validation block (lines 51-81), mirror an existing check:

```rust
    if form.reseed_requestable_after_hours < 1 {
        return Err(Error::InvalidInput(
            "reseed_requestable_after_hours must be at least 1".into(),
        ));
    }
```

(Use whatever error constructor the surrounding validations use — match the existing block exactly.)

- [ ] **Step 10: Verify compile/lint and commit**

Run: `cargo clippy -p arcadia-storage -p arcadia-api` (and `cargo test` if a dev DB is available)
Expected: no new warnings/errors (or note tests pending DB rebuild).

```bash
git add backend/storage/src/models/user.rs backend/storage/src/models/arcadia_settings.rs backend/storage/src/repositories/arcadia_settings_repository.rs backend/api/src/handlers/arcadia_settings/update_arcadia_settings.rs
git commit -m "feat: add RequestReseed permission and reseed_requestable_after_hours setting"
```

### Notification type

- [ ] **Step 11: Add the NotificationEvent variant + response structs**

In `backend/storage/src/models/notification.rs`:
- Add the enum variant (lines 10-21):

```rust
    ReseedRequest { user_ids: Vec<i32> },
```

- Add its arm to `user_ids()` (24-37): `NotificationEvent::ReseedRequest { user_ids } => user_ids.clone(),`
- Add its arm to `event_type()` (39-52): `NotificationEvent::ReseedRequest { .. } => "reseed_request",`
- Add a response struct mirroring `NotificationTitleGroupTorrent` (line 89), carrying the torrent + title-group linking fields it uses, plus `requested_by` and `created_at`:

```rust
#[derive(Debug, Serialize, ToSchema, FromRow)]
pub struct NotificationReseedRequest {
    pub torrent_id: i32,
    pub title_group_id: i32,
    pub title_group_name: String,
    pub requested_by: Option<i32>,
    pub created_at: DateTime<Utc>,
}
```

(Match the exact field set and types that `NotificationTitleGroupTorrent` uses for linking; add/remove to align with the SELECT in Step 13.)

- Add a field to `Notifications` (189-201): `pub reseed_requests: Vec<NotificationReseedRequest>,`
- Add a field to `NotificationCounts` (173-187): `pub reseed_requests: i64,`

- [ ] **Step 12: Add the recipient-insert repo function**

In `backend/storage/src/repositories/notification_repository.rs`, add near `notify_users_title_group_torrents` (382):

```rust
pub async fn notify_users_reseed_request(
    &self,
    torrent_id: i32,
    requested_by: i32,
) -> Result<Vec<i32>> {
    let rows = sqlx::query_scalar!(
        r#"
        INSERT INTO notifications_reseed_requests (torrent_id, user_id, requested_by)
        SELECT ta.torrent_id, ta.user_id, $2
        FROM torrent_activities ta
        WHERE ta.torrent_id = $1
          AND ta.user_id <> $2
          AND (ta.first_seen_seeding_at IS NOT NULL OR ta.completed_at IS NOT NULL)
          AND NOT EXISTS (
              SELECT 1 FROM notifications_reseed_requests n
              WHERE n.torrent_id = ta.torrent_id
                AND n.user_id = ta.user_id
                AND n.read_status = FALSE
          )
        RETURNING user_id
        "#,
        torrent_id,
        requested_by,
    )
    .fetch_all(&self.pool)
    .await?;
    Ok(rows)
}
```

(Adjust `self.pool` / return wrapping to match the surrounding functions' exact style — e.g. whether they take a transaction. Follow `notify_users_title_group_torrents`.)

- [ ] **Step 13: Add the fetch + count SQL**

In `find_all_notifications` (20-281), add a SELECT block for reseed requests (filter `read_status = FALSE` unless `include_read`), mirroring the title-group-torrent block and joining to `torrents`/`title_groups` for the linking fields in `NotificationReseedRequest`. In `find_notification_counts` (1011-1090), add a count subquery:

```sql
    (SELECT COUNT(*) FROM notifications_reseed_requests WHERE user_id = $1 AND read_status = FALSE) AS "reseed_requests!",
```

Populate the new `reseed_requests` fields on `Notifications` and `NotificationCounts`.

- [ ] **Step 14: Verify compile/lint and commit**

Run: `cargo clippy -p arcadia-storage` (+ `cargo test` if DB available)

```bash
git add backend/storage/src/models/notification.rs backend/storage/src/repositories/notification_repository.rs
git commit -m "feat: add reseed_request notification type plumbing"
```

### Trigger endpoint (TDD)

- [ ] **Step 15: Write the failing integration test**

Create `backend/api/tests/test_reseed_request.rs`. Follow an existing test in `backend/api/tests/` for app/pool setup and the `with_test_users_with_permissions` fixture. Cover the Review Focus cases:

```rust
// Pseudocode shape — adapt harness setup to an existing test file in backend/api/tests/.
#[sqlx::test(fixtures("with_test_users_with_permissions"))]
async fn reseed_request_notifies_past_seeders_and_snatchers(pool: PgPool) {
    // Arrange: a torrent with seeders=0; torrent_activities rows:
    //   user A: first_seen_seeding_at = now()-100h (past seeder)
    //   user B: completed_at = now()-100h, no seeding (snatcher)
    //   user C (requester): also a past seeder
    // Set arcadia_settings.reseed_requestable_after_hours = 72.
    // Act: POST /api/torrents/{id}/reseed-request as user C (has request_reseed perm).
    // Assert: 200; notifications_reseed_requests has rows for A and B, NOT C.
}

#[sqlx::test(fixtures("with_test_users_with_permissions"))]
async fn reseed_request_rejected_when_torrent_has_seeders(pool: PgPool) {
    // torrent seeders=2 -> POST returns eligibility error; no rows inserted.
}

#[sqlx::test(fixtures("with_test_users_with_permissions"))]
async fn reseed_request_rejected_when_not_dead_long_enough(pool: PgPool) {
    // last_seen_seeding_at = now()-1h, threshold 72 -> rejected; no rows.
}

#[sqlx::test(fixtures("with_test_users_with_permissions"))]
async fn reseed_request_rejected_when_never_seeded(pool: PgPool) {
    // no torrent_activities seeding/completed timestamps -> MAX is NULL -> rejected.
}

#[sqlx::test(fixtures("with_test_users_with_permissions"))]
async fn reseed_request_forbidden_without_permission(pool: PgPool) {
    // requester lacks request_reseed -> 403, no rows.
}

#[sqlx::test(fixtures("with_test_users_with_permissions"))]
async fn reseed_request_does_not_duplicate_unread(pool: PgPool) {
    // Two successive POSTs -> recipient has exactly one unread row.
}
```

- [ ] **Step 16: Run test to verify it fails**

Run: `cargo test -p arcadia-api --test test_reseed_request`
Expected: FAIL (endpoint/route not found). If no dev DB, note that compilation of the new route must come first; proceed to implement then rerun.

- [ ] **Step 17: Implement the handler**

Create `backend/api/src/handlers/torrents/request_reseed.rs`, following `create_torrent.rs` structure:

```rust
#[utoipa::path(
    post,
    operation_id = "Request reseed",
    tag = "Torrents",
    path = "/api/torrents/{id}/reseed-request",
    params(("id" = i32, Path, description = "Torrent id")),
    responses((status = 200, description = "Reseed requested")),
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    path: Path<i32>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
    req: HttpRequest,
) -> Result<HttpResponse> {
    arc.pool
        .require_permission(user.sub, &UserPermission::RequestReseed, req.path())
        .await?;

    let torrent_id = path.into_inner();
    let threshold_hours = arc.settings.lock().unwrap().reseed_requestable_after_hours;

    // Single query: fetch current seeders and the global last-seeded time, then decide.
    let row = sqlx::query!(
        r#"
        SELECT t.seeders AS seeders,
               (SELECT MAX(ta.last_seen_seeding_at)
                FROM torrent_activities ta WHERE ta.torrent_id = t.id) AS last_seeded_at
        FROM torrents t WHERE t.id = $1
        "#,
        torrent_id
    )
    .fetch_optional(&arc.pool.pool)
    .await?
    .ok_or(Error::NotFound)?;

    if row.seeders != 0 {
        return Err(Error::ReseedNotEligible); // torrent still has seeders
    }
    let last_seeded = row.last_seeded_at.ok_or(Error::ReseedNotEligible)?;
    if Utc::now() - last_seeded < Duration::hours(threshold_hours as i64) {
        return Err(Error::ReseedNotEligible);
    }

    let user_ids = arc
        .pool
        .notify_users_reseed_request(torrent_id, user.sub)
        .await?;
    if !user_ids.is_empty() {
        let _ = arc
            .notification_sender
            .send(NotificationEvent::ReseedRequest { user_ids });
    }
    Ok(HttpResponse::Ok().finish())
}
```

Adapt: the exact pool accessor (`&arc.pool.pool` vs a helper), the `Error` variant names (add a `ReseedNotEligible` variant to `backend/common/src/error.rs` if none fits — mirror an existing `BadRequest`-style variant with an HTTP 409/400 mapping), and `Duration`/`Utc` imports (`chrono`).

- [ ] **Step 18: Register the route, doc, and scope**

- In `backend/api/src/handlers/torrents/mod.rs`, add `pub mod request_reseed;` and a route on the torrents scope: `.service(resource("/{id}/reseed-request").route(post().to(request_reseed::exec::<R>)))` (match the module's existing resource style).
- In `backend/api/src/api_doc.rs`, add `request_reseed::exec` to the paths list.
- In `backend/api/src/middlewares/api_key_scopes.rs`, add the new path to the appropriate scope entry (mirror a neighboring torrents write path).

- [ ] **Step 19: Run tests to verify they pass**

Run: `cargo test -p arcadia-api --test test_reseed_request`
Expected: PASS (requires dev DB; if unavailable, run `cargo clippy -p arcadia-api` and note tests pending the author's DB rebuild).

- [ ] **Step 20: Commit**

```bash
git add backend/api/src/handlers/torrents/request_reseed.rs backend/api/src/handlers/torrents/mod.rs backend/api/src/api_doc.rs backend/api/src/middlewares/api_key_scopes.rs backend/common/src/error.rs backend/api/tests/test_reseed_request.rs
git commit -m "feat: add reseed request endpoint with eligibility checks"
```

### Auto-clear job (TDD)

- [ ] **Step 21: Write the failing test**

Add to `backend/api/tests/test_reseed_request.rs` a test of the delete query (exercise the SQL directly against the pool, since the job is a thin wrapper):

```rust
#[sqlx::test(fixtures("with_test_users_with_permissions"))]
async fn auto_clear_deletes_only_healthy_torrents(pool: PgPool) {
    // torrent H: seeders=1, leechers=0  -> its reseed notifs must be deleted
    // torrent L: seeders=1, leechers=3  -> kept
    // torrent D: seeders=0              -> kept
    // Insert one reseed notif per torrent, run the DELETE from Step 22, assert counts.
}
```

- [ ] **Step 22: Implement the job**

Create `backend/periodic-tasks/src/periodic_tasks/reseed_requests.rs`, mirroring `announce_errors.rs`:

```rust
use std::sync::Arc;
use arcadia_storage::connection_pool::ConnectionPool;
use arcadia_common::error::Result;

pub async fn cleanup_resolved_reseed_requests(pool: Arc<ConnectionPool>) -> Result<()> {
    sqlx::query!(
        r#"
        DELETE FROM notifications_reseed_requests
        WHERE torrent_id IN (
            SELECT id FROM torrents WHERE seeders >= 1 AND leechers = 0
        )
        "#
    )
    .execute(&pool.pool)
    .await?;
    Ok(())
}
```

(Match `announce_errors.rs` for the exact signature, pool accessor, and return type.)

- [ ] **Step 23: Wire config + scheduler**

- `backend/periodic-tasks/src/periodic_tasks/mod.rs`: add `pub mod reseed_requests;`.
- `backend/periodic-tasks/src/config.rs` (`PeriodicTasksConfig`, lines 10-31): add `pub reseed_request_cleanup_seconds: u64,` with a sensible default consistent with how other intervals get defaults.
- `backend/periodic-tasks/src/periodic_tasks/scheduler.rs`: register a `Job::new_repeated_async(Duration::from_secs(config.reseed_request_cleanup_seconds), ...)` block mirroring `announce_errors_cleanup` (line ~157), wrapped in `instrument_periodic_task`, calling `reseed_requests::cleanup_resolved_reseed_requests(pool.clone())`.
- `config.example.yml`: add `reseed_request_cleanup_seconds:` under the `periodic_tasks:` section with the default value.

- [ ] **Step 24: Run tests + lint**

Run: `cargo test -p arcadia-api --test test_reseed_request` and `cargo clippy -p arcadia-periodic-tasks`
Expected: PASS / clean (or note tests pending DB rebuild).

- [ ] **Step 25: Commit**

```bash
git add backend/periodic-tasks/ config.example.yml backend/api/tests/test_reseed_request.rs
git commit -m "feat: auto-clear reseed notifications when torrent is healthy"
```

### Display field

- [ ] **Step 26: Add `last_seeded_at` to TorrentHierarchy**

In `backend/storage/src/models/torrent.rs` (`TorrentHierarchy`, ~line 613, near `seeders`/`leechers`):

```rust
    pub last_seeded_at: Option<DateTime<Utc>>,
```

- [ ] **Step 27: Populate it in the query + build loop**

In `backend/storage/src/repositories/title_group_repository.rs`:
- In the torrents SELECT (164-196), add to the column list:

```sql
    CASE WHEN t.seeders = 0
         THEN (SELECT MAX(ta.last_seen_seeding_at)
               FROM torrent_activities ta WHERE ta.torrent_id = t.id)
    END AS last_seeded_at
```

- In the `TorrentHierarchy` build loop (455-533), set `last_seeded_at: row.last_seeded_at,` (match how adjacent fields are read from the row).

- [ ] **Step 28: Verify + commit**

Run: `cargo clippy -p arcadia-storage` (+ `cargo test` if DB available)

```bash
git add backend/storage/src/models/torrent.rs backend/storage/src/repositories/title_group_repository.rs
git commit -m "feat: expose last_seeded_at on dead torrents in title group view"
```

- [ ] **Step 29: Full backend verification**

Run: `cargo clippy --workspace` and, with a dev DB, `cargo test --workspace` (at minimum `-p arcadia-api`).
Expected: clean. If no DB, explicitly report: "Backend compiles via clippy; `cargo test` pending author DB + `.sqlx` rebuild."

---

## Task 2: Frontend — settings, torrent-row display + button, notification UI

**Files:**
- Regenerate (author/CI, do not hand-edit): `frontend/src/services/api-schema/` from the updated OpenAPI spec.
- Modify: the title-group torrent-row component (the component rendering an expanded torrent under a release name on the title-group page — locate under `frontend/src/components/` by searching for where `TorrentHierarchy` seeders/leechers are rendered).
- Modify: `frontend/src/services/notificationStream.ts:10-19` (`eventTypeToCountKey`)
- Modify: `frontend/src/stores/notifications.ts:5-18` (count field)
- Create: `frontend/src/components/notification/ReseedRequestsNotifications.vue`
- Modify: `frontend/src/views/NotificationsView.vue` (mount the new component alongside the others)
- Modify: i18n locale files under `frontend/src/i18n/` (button label, notification text, last-seeded label)
- Reference: an existing per-type component, e.g. `frontend/src/components/notification/TitleGroupTorrentsNotifications.vue`

**Interfaces:**
- Consumes (from Task 1, via regenerated client): `POST /api/torrents/{id}/reseed-request`; `TorrentHierarchy.lastSeededAt?: string`; `PublicArcadiaSettings.reseedRequestableAfterHours: number`; `NotificationCounts.reseedRequests: number`; `Notifications.reseedRequests`; SSE event `"reseed_request"`.

- [ ] **Step 1: Regenerate the API client**

After Task 1's OpenAPI changes are live, regenerate `frontend/src/services/api-schema/` (the project's generate script / CI step). Do not hand-edit. Confirm the new operation and fields appear (camelCase: `lastSeededAt`, `reseedRequestableAfterHours`, `reseedRequests`).

- [ ] **Step 2: Add the SSE → count-key mapping**

In `frontend/src/services/notificationStream.ts`, add to `eventTypeToCountKey` (10-19):

```ts
  reseed_request: 'reseedRequests',
```

- [ ] **Step 3: Add the store count field**

In `frontend/src/stores/notifications.ts` (5-18), add `reseedRequests: 0,` to the counts state shape, mirroring the other count fields.

- [ ] **Step 4: Add i18n keys**

In each locale file under `frontend/src/i18n/`, add keys mirroring an existing notification type, e.g.:

```
reseed_request.button: "Request reseed"
reseed_request.last_seeded: "Last seeded {date}"
reseed_request.notification: "A reseed was requested for {torrent}"
```

(Match the project's actual i18n key structure and the existing notification key naming.)

- [ ] **Step 5: Build the notification component**

Create `frontend/src/components/notification/ReseedRequestsNotifications.vue` by copying `TitleGroupTorrentsNotifications.vue` and adapting it to iterate `notifications.reseedRequests`, link each entry to its release (`titleGroupId`), and use the `reseed_request.*` i18n keys. Mount it in `frontend/src/views/NotificationsView.vue` next to the existing per-type components.

- [ ] **Step 6: Add last-seeded date + reseed button to the torrent row**

In the title-group torrent-row component, where a torrent is expanded under the release name, add (PrimeVue + Pinia, using the generated client — no raw axios):

```vue
<template v-if="torrent.seeders === 0 && torrent.lastSeededAt">
  <div class="reseed-info">
    <span>{{ t('reseed_request.last_seeded', { date: formatDate(torrent.lastSeededAt) }) }}</span>
    <Button
      v-if="canRequestReseed && isReseedEligible(torrent)"
      :label="t('reseed_request.button')"
      size="small"
      @click="requestReseed(torrent.id)"
    />
  </div>
</template>
```

```ts
// canRequestReseed: user permissions include 'request_reseed' (use the existing permission-check helper/composable).
// isReseedEligible(torrent): compares against the public setting.
function isReseedEligible(torrent): boolean {
  if (torrent.seeders !== 0 || !torrent.lastSeededAt) return false
  const hours = settings.reseedRequestableAfterHours // from public settings store
  const deadMs = Date.now() - new Date(torrent.lastSeededAt).getTime()
  return deadMs >= hours * 3600 * 1000
}
async function requestReseed(id: number) {
  await torrentsApi.requestReseed(id) // generated client method; name per OpenAPI operationId
  // toast success using existing toast util
}
```

Use the project's existing permission-check and public-settings access patterns (find how another button is gated by a `UserPermission` on the frontend and how `PublicArcadiaSettings` is read).

- [ ] **Step 7: Verify**

Run: `npm run lint` and `npm run type-check` (or the project's equivalents, see `frontend/package.json`) and build.
Expected: clean.

- [ ] **Step 8: Commit**

```bash
git add frontend/
git commit -m "feat: reseed request UI — button, last-seeded date, notifications"
```

---

## Notes for the executor

- The DB-dependent steps (`cargo test`) assume a running dev Postgres with `DATABASE_URL`. If absent, implement + `cargo clippy`, and clearly report that `cargo test` and the `.sqlx` cache regeneration are pending the author. Never run `cargo sqlx prepare`.
- Field names in `NotificationReseedRequest` (Step 11) and its SELECT (Step 13) must match exactly — align both to whatever linking fields `NotificationTitleGroupTorrent` actually uses once you read that struct.
- The frontend `operationId` → client method name mapping comes from the regenerated client; use the actual generated name, not a guessed one.
