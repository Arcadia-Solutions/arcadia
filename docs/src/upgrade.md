# Upgrading (Bare-Metal)

If you deployed using Docker, see the [Docker Upgrading Guide](run-docker.md#upgrading).

---

Updating a bare-metal installation involves pulling new Git commits, recompiling binaries, and rebuilding frontend assets.

Because Arcadia is currently under rapid development, database schema changes are often committed directly to the baseline migration (`backend/storage/migrations/20250312215600_initdb.sql`) rather than distributed as incremental migration scripts. When pulling updates that modify the database schema, follow the migration and restore procedure below.

---

## 1. Routine Updates (No Schema Changes)

If the new commits do not modify files in `backend/storage/migrations/`:

```bash
git fetch && git pull
cargo build -p arcadia-api --release
cargo build -p arcadia_tracker --release
cd frontend && npm install && npm run build && cd ..
```

Restart your backend (`arcadia-api`) and tracker (`arcadia_tracker`) processes or systemd services.

---

## 2. Upgrading Across Schema Changes

When upstream commits alter `20250312215600_initdb.sql`, running `sqlx migrate run` directly against an existing database will fail with a checksum mismatch error. Use the following procedure to migrate your data into the updated schema.

### Step 1: Dump Existing Data

Create a data-only SQL dump with explicit column inserts, excluding the `_sqlx_migrations` table:

```bash
pg_dump -U arcadia -d arcadia --data-only --column-inserts -T _sqlx_migrations > arcadia-data.sql
```

> [!IMPORTANT]
> Store `arcadia-data.sql` in a safe backup location before proceeding.

### Step 2: Pull Latest Commits

```bash
git fetch && git pull
```

### Step 3: Recreate Database and Apply Schema

Drop and recreate the database, then run the updated schema migration:

```bash
dropdb -U arcadia arcadia
createdb -U arcadia arcadia
cd backend/storage
sqlx migrate run --database-url postgresql://arcadia:your_password@localhost:5432/arcadia
cd ../..
```

### Step 4: Clear Pre-seeded Default Rows

The initial migration seeds default rows (`arcadia_settings`, system `users`, default `user_classes`, `css_sheets`, and initial `forum_*` entries). Truncate these seeded tables before restoring your dump to prevent primary key conflicts:

```bash
psql -U arcadia -d arcadia -c "
TRUNCATE users, user_classes, css_sheets, arcadia_settings, 
         forum_categories, forum_sub_categories, forum_threads, forum_posts CASCADE;
"
```

### Step 5: Restore Data

Restore the dump while temporarily disabling foreign-key triggers (`session_replication_role = 'replica'`). This allows foreign-keyed tables (such as hierarchical `user_classes` or forum relationships) to restore regardless of insert ordering:

```bash
psql -U arcadia -d arcadia -v ON_ERROR_STOP=1 <<EOF
SET session_replication_role = 'replica';
\i arcadia-data.sql
SET session_replication_role = 'default';
EOF
```

### Step 6: Recompile and Restart

Recompile the backend binaries and frontend assets:

```bash
cargo build -p arcadia-api --release
cargo build -p arcadia_tracker --release
cd frontend && npm install && npm run build && cd ..
```

Restart your services to run the updated binaries.
