# Reseed Request — Design Spec

Date: 2026-10-02
Status: Approved design, pending implementation plan

## Problem & Intent

When a torrent loses all its seeders, the content becomes undownloadable. We
want users to be able to **ping everyone who ever helped carry that torrent**
(past seeders and past snatchers) and ask them to start seeding again — but
only once the torrent has genuinely been dead for a while, so the feature is a
rescue signal and not a nag.

This is delivered as a **new in-app notification type** (`reseed_request`) that:

- is triggered manually by a user holding a new `request_reseed` permission,
- is only allowed when the torrent has had **zero seeders for at least a
  configurable number of hours**,
- is sent to every user who ever seeded **or** snatched that torrent,
- is **auto-cleared (hard-deleted)** once the torrent is healthy again, defined
  as `seeders >= 1 AND leechers = 0`.

### Success criteria

- A permitted user viewing a dead torrent sees when it was last seeded and a
  "request reseed" button, and clicking it notifies the right set of users.
- The request is rejected server-side if the torrent is not actually eligible
  (still has seeders, or not dead long enough), regardless of what the client
  shows.
- Recipients see the notification via the existing SSE + notification UI, and it
  disappears on its own once the torrent recovers.

## Constraints & Existing Patterns

Grounded in the current codebase (all paths canonical; ignore
`.claude/worktrees/` copies):

- **No incremental migrations.** All schema changes go directly into the single
  `backend/storage/migrations/20250312215600_initdb.sql` (per `backend/CLAUDE.md`).
- **Do not run `cargo sqlx prepare` or build.** After schema edits, the author
  rebuilds the DB and the `.sqlx` offline cache. We verify only with
  `cargo clippy` and `cargo test`.
- **Notifications are per-type**, not generic: each type has its own
  `notifications_<x>` table, its own `NotificationEvent` enum variant, its own
  insert/count/fetch/mark-read SQL, and its own Vue component. Reference shapes:
  `notifications_title_group_torrents` (opt-in via a `subscriptions_*` table) and
  `torrent_deletions` (recipients computed directly, no subscription table).
- **Auto-clear precedent:** `announce_errors` rows are hard-DELETEd by a periodic
  task (`remove_resolved_and_stale_announce_errors`) once resolved. We follow
  this model.
- **No "seeders hit zero" timestamp exists.** The authoritative source for "when
  did this torrent last have a seeder" is
  `MAX(torrent_activities.last_seen_seeding_at)` across all users for that
  torrent. Current counts are the denormalized `torrents.seeders` /
  `torrents.leechers` columns, maintained by the tracker.
- **Recipients source:** `torrent_activities` rows where
  `first_seen_seeding_at IS NOT NULL` (ever seeded) or `completed_at IS NOT NULL`
  (ever snatched/finished).
- **Scheduled jobs:** `backend/periodic-tasks` crate, `tokio-cron-scheduler`,
  config-driven intervals in `config.rs`, registered in `scheduler.rs`.
- **Settings:** `arcadia_settings` is a single-row, typed-column table, cached in
  memory as `arc.settings` (`Mutex<ArcadiaSettings>`); a `PublicArcadiaSettings`
  subset is exposed unauthenticated.
- **Permissions:** `UserPermission` enum (`sqlx::Type` + `ToSchema`, auto-exposed
  to OpenAPI) backed by the `user_permissions_enum` Postgres type; checked with
  `arc.pool.require_permission(user_id, &UserPermission::X, path)`.

## Design

### 1. Database (`20250312215600_initdb.sql`)

**1a. New permission literal** — add `'request_reseed'` to the
`user_permissions_enum` `CREATE TYPE` block.

**1b. New setting column** on `arcadia_settings`:

```sql
reseed_requestable_after_hours INT NOT NULL DEFAULT 72
```

`NOT NULL DEFAULT` means the single-row seed `INSERT` does not need editing.

**1c. New notification table** (per-type shape; no subscription table — recipients
are computed like `torrent_deletions`):

```sql
CREATE TABLE notifications_reseed_requests (
    id           BIGSERIAL PRIMARY KEY,
    torrent_id   INT NOT NULL REFERENCES torrents(id) ON DELETE CASCADE,
    user_id      INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,  -- recipient
    requested_by INT REFERENCES users(id) ON DELETE SET NULL,          -- who asked
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    read_status  BOOLEAN NOT NULL DEFAULT FALSE
);
CREATE INDEX idx_notifications_reseed_requests_user ON notifications_reseed_requests(user_id, read_status);
CREATE INDEX idx_notifications_reseed_requests_torrent ON notifications_reseed_requests(torrent_id);
```

Notes:
- **No `UNIQUE(torrent_id, user_id)`.** De-duplication is done the same way as
  every other type: the insert carries a `WHERE NOT EXISTS (... read_status = FALSE)`
  guard, so a fresh request can be raised again after a prior one is cleared.
- `read_status` is retained only so the standard per-type count/fetch SQL
  templates work unchanged; it stays `FALSE` in practice (there is no
  mark-as-read path — see §6 and §7). A manual-dismiss endpoint is a trivial
  future extension if wanted.

### 2. Permission

- Add `RequestReseed` variant to `UserPermission`
  (`backend/storage/src/models/user.rs`), mapping to `'request_reseed'`.
- Checked in the trigger handler via
  `arc.pool.require_permission(user.sub, &UserPermission::RequestReseed, req.path()).await?`.
- No central permission list to edit — `ToSchema` auto-publishes the variant;
  assignment is data-driven via `users.permissions` / `user_classes.new_permissions`.

### 3. Setting plumbing

- Add `reseed_requestable_after_hours: i32` to `ArcadiaSettings`
  **and** to `PublicArcadiaSettings` (the client compares against it)
  in `backend/storage/src/models/arcadia_settings.rs`.
- Add the column to all three query sites in
  `backend/storage/src/repositories/arcadia_settings_repository.rs`:
  the `SELECT` in `get_arcadia_settings`, and the `UPDATE SET` list +
  `RETURNING` list + bind args in `update_arcadia_settings` (mind the sequential
  `$N` numbering).
- Add a range validation block (`>= 1`) in
  `backend/api/src/handlers/arcadia_settings/update_arcadia_settings.rs`,
  mirroring existing numeric validations.

### 4. Trigger endpoint

`POST /api/torrents/{id}/reseed-request` — new handler under
`backend/api/src/handlers/torrents/`, route added to the torrents feature
`mod.rs`, documented in `backend/api/src/api_doc.rs`, and its path added to the
appropriate scope in `backend/api/src/middlewares/api_key_scopes.rs`.

Handler logic:

1. `require_permission(RequestReseed)`.
2. Load the torrent. Reject with a domain error if `seeders != 0`
   (`ReseedNotEligibleHasSeeders` or equivalent).
3. Compute `last_seeded_at = MAX(torrent_activities.last_seen_seeding_at)` for the
   torrent. Reject if it is `NULL` or
   `now() - last_seeded_at < reseed_requestable_after_hours`
   (read the threshold from `arc.settings.lock().unwrap()`).
4. Insert reseed notifications for all recipients and broadcast.

Repository function `notify_users_reseed_request(torrent_id, requested_by)` in
`backend/storage/src/repositories/notification_repository.rs`:

```sql
INSERT INTO notifications_reseed_requests (torrent_id, user_id, requested_by)
SELECT ta.torrent_id, ta.user_id, $2
FROM torrent_activities ta
WHERE ta.torrent_id = $1
  AND ta.user_id <> $2                               -- exclude the requester
  AND (ta.first_seen_seeding_at IS NOT NULL OR ta.completed_at IS NOT NULL)
  AND NOT EXISTS (
      SELECT 1 FROM notifications_reseed_requests n
      WHERE n.torrent_id = ta.torrent_id
        AND n.user_id = ta.user_id
        AND n.read_status = FALSE
  )
RETURNING user_id;
```

The handler then broadcasts
`NotificationEvent::ReseedRequest { user_ids }` via `arc.notification_sender`
(the `RETURNING user_id` list).

### 5. Notification plumbing (mirror an existing type end to end)

In `backend/storage/src/models/notification.rs`:
- `NotificationEvent::ReseedRequest { user_ids: Vec<i32> }` + its arm in
  `user_ids()` and `event_type()` (→ `"reseed_request"`).
- `NotificationReseedRequest` response struct: torrent id + the title-group
  linking info the other torrent-related notifications carry (so the UI can link
  to the release), `requested_by`, `created_at`.
- Add a field to the `Notifications` aggregate and a count field to
  `NotificationCounts`.

In `backend/storage/src/repositories/notification_repository.rs`:
- A `SELECT` block in `find_all_notifications` (filtered `read_status = FALSE`,
  or included when `include_read`).
- A count subquery in `find_notification_counts`.
- `notify_users_reseed_request` (see §4). **No** `mark_..._as_read` function and
  **no** page-view auto-mark (viewing the title group must not silently clear a
  reseed request — clearing is condition-based only, see §7).

### 6. Auto-clear periodic job

- New task file `backend/periodic-tasks/src/periodic_tasks/reseed_requests.rs`
  plus `mod` entry.
- New config field `reseed_request_cleanup_seconds` in
  `backend/periodic-tasks/src/config.rs` (`PeriodicTasksConfig`) and
  `config.example.yml`.
- Register a `Job::new_repeated_async(...)` block in
  `backend/periodic-tasks/src/periodic_tasks/scheduler.rs`, wrapped in
  `instrument_periodic_task`, mirroring `announce_errors_cleanup`.

Query (**hard delete**, using the cheap cached counters):

```sql
DELETE FROM notifications_reseed_requests
WHERE torrent_id IN (
    SELECT id FROM torrents WHERE seeders >= 1 AND leechers = 0
);
```

### 7. Display (title-group page only)

- Add `last_seeded_at: Option<DateTime<Utc>>` to `TorrentHierarchy`
  (`backend/storage/src/models/torrent.rs`). Not added to
  `TorrentHierarchyLite` — search/browse is out of scope.
- Populate it in `find_title_group_hierarchy`
  (`backend/storage/src/repositories/title_group_repository.rs`), in the torrents
  `SELECT` (the `t.seeders, t.leechers` block), computed cheaply so the subquery
  only runs for dead torrents:

```sql
CASE WHEN t.seeders = 0
     THEN (SELECT MAX(ta.last_seen_seeding_at)
           FROM torrent_activities ta
           WHERE ta.torrent_id = t.id)
END AS last_seeded_at
```

  Map it into the `TorrentHierarchy` struct in the build loop. This aggregate is
  **not** user-filtered (unlike the existing per-user `torrent_activities` query
  used for `peer_status`).

Frontend (Vue 3 / PrimeVue):
- When a torrent row is expanded on the title-group page, render under the
  release name: the `last_seeded_at` date, and — when
  `torrent.seeders === 0 && last_seeded_at && (now - last_seeded_at) >= reseed_requestable_after_hours (in seconds) && user has request_reseed permission` —
  a "Request reseed" button calling `POST /api/torrents/{id}/reseed-request`.
- `reseed_requestable_after_hours` comes from the public settings already fetched
  by the client. The client check is UX only; the backend (§4) is authoritative.

Notification UI, mirroring an existing per-type component:
- `eventTypeToCountKey` entry (`reseed_request`) in
  `frontend/src/services/notificationStream.ts`.
- A count field in `frontend/src/stores/notifications.ts`.
- A `ReseedRequestsNotifications.vue` component under
  `frontend/src/components/notification/`, linking each item to its release.
- i18n keys for the button, the notification text, and the last-seeded label.
- The API client in `frontend/src/services/api-schema/` regenerates from the
  OpenAPI spec (do not hand-edit).

### 8. Testing

Actix integration tests under `backend/api/tests/`, using the
`with_test_users_with_permissions` fixture pattern:

- **Eligible + permission** → notifications are created for exactly the
  seeders ∪ snatchers set, excluding the requester; `NotificationEvent` broadcast.
- **Torrent still has seeders** → rejected (no rows created).
- **Dead but not long enough** (`last_seen_seeding_at` within the threshold) →
  rejected.
- **No permission** → `403` / `InsufficientPermissions`.
- **Re-trigger while an unread request exists** → no duplicate rows (dedup guard).
- **Auto-clear job**: with a torrent at `seeders >= 1 AND leechers = 0`, the job
  deletes its reseed notifications; a torrent with leechers still present keeps
  them.

Verify with `cargo clippy` and `cargo test` only. Do **not** run
`cargo sqlx prepare` or a full build — flag to the author that the DB + `.sqlx`
cache need rebuilding after the `initdb.sql` changes.

## Out of Scope (YAGNI)

- Email / push delivery of reseed requests (in-app + SSE only; the email service
  also lives in the `api` crate and is not reachable from `periodic-tasks`).
- A manual "dismiss reseed request" endpoint / mark-as-read path.
- Showing `last_seeded_at` or the button on search/browse grids
  (`TorrentHierarchyLite`).
- Any rate limiting or cooldown on the trigger action beyond the row-level unread
  dedup (explicitly requested: no limits beyond the permission).
- Automatic (system-initiated) reseed requests — the trigger is always a
  permitted user's action.

## Touch-point Summary

| Area | Files |
|------|-------|
| Schema | `backend/storage/migrations/20250312215600_initdb.sql` (permission enum, settings column, new table) |
| Permission | `backend/storage/src/models/user.rs` |
| Settings | `backend/storage/src/models/arcadia_settings.rs`, `backend/storage/src/repositories/arcadia_settings_repository.rs`, `backend/api/src/handlers/arcadia_settings/update_arcadia_settings.rs` |
| Trigger | new handler in `backend/api/src/handlers/torrents/`, torrents `mod.rs`, `backend/api/src/api_doc.rs`, `backend/api/src/middlewares/api_key_scopes.rs` |
| Notification | `backend/storage/src/models/notification.rs`, `backend/storage/src/repositories/notification_repository.rs` |
| Auto-clear | `backend/periodic-tasks/src/periodic_tasks/reseed_requests.rs` (+ `mod.rs`), `backend/periodic-tasks/src/config.rs`, `backend/periodic-tasks/src/periodic_tasks/scheduler.rs`, `config.example.yml` |
| Display | `backend/storage/src/models/torrent.rs` (`TorrentHierarchy`), `backend/storage/src/repositories/title_group_repository.rs` |
| Frontend | `frontend/src/services/notificationStream.ts`, `frontend/src/stores/notifications.ts`, `frontend/src/components/notification/ReseedRequestsNotifications.vue`, title-group torrent-row component, i18n |
| Tests | `backend/api/tests/` |
