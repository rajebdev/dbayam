# DBayam architecture

DBayam is a single native process. The UI thread (egui/eframe, glow renderer)
only draws state and sends commands; all database I/O, metadata loading and
data transfer runs on a small tokio runtime (2–4 worker threads). The UI polls
finished background jobs once per frame and repaints only when something
changed, so the application uses ~0% CPU while idle.

```
┌──────────────────────────── dbayam-app (binary "dbayam") ────────────────────────────┐
│ UI thread (egui)                                                                     │
│  navigator · SQL editor tabs · result grids · object editors · diagrams · admin      │
│  tools · dialogs · command palette · status bar · operations panel                   │
│        │ commands / jobs                                   ▲ results (Job<T>)        │
│        ▼                                                   │                         │
│ Backend (tokio runtime) ── ConnectionManager ── SessionHandle (1 open cursor/session)│
└────────┬───────────────────────────┬───────────────────────────────┬─────────────────┘
         │                           │                               │
   dbayam-sql (tokenizer,      dbayam-core (Value, RowStore,  dbayam-net (SSH tunnel,
   splitter, formatter,        Driver/Session traits,         SOCKS/HTTP proxy as a
   autocomplete, analysis)     metadata/DDL/plan model,       local forwarder)
                               Dialect, i18n, Secret)
         │                           │
   dbayam-grid / dbayam-editor drivers: postgres (+CockroachDB), mysql (+MariaDB),
   dbayam-visual (egui         sqlite, mssql (tiberius), clickhouse (HTTP), duckdb
   widgets)                    (optional), mongodb (shell language), redis
         │
   dbayam-config (settings, connections, secrets: Keychain / encrypted vault,
   DBeaver / pgpass / URL import, query history, projects & scripts)
   dbayam-transfer (export/import/migration/mock data) · dbayam-compare (schema &
   data compare, HTML, Liquibase) · dbayam-tasks (tasks, scheduler) · dbayam-ai
```

## Crates

| Crate | Responsibility |
| --- | --- |
| `dbayam-core` | Shared contract: `Value`/`DataKind`/`ColumnMeta`, `Driver`/`Session`/`QueryCursor` traits, metadata, DDL and plan models, `Dialect` + ANSI DDL helpers, `RowStore` (paged, spills to disk), `Secret`, i18n. |
| `dbayam-sql` | Dialect-aware tokenizer, statement splitter (DELIMITER, GO, compound blocks, dollar quotes), classifier (read-only enforcement, dangerous statements), formatter, autocomplete and semantic analysis, parameters, client commands, outline, spell checker, SQL generation, fuzzy matching. |
| `dbayam-driver-*` | One crate per product. Each implements `Driver`, `Session`, `QueryCursor`, `Dialect` and `DdlGenerator` and reads metadata from the system catalogs. MongoDB and Redis set `SqlSyntax::shell`: `dbayam-sql` then splits and classifies their statements with the shell rules instead of SQL, and `Dialect::table_data_query` gives the data editor their own read statement. |
| `dbayam-net` | SSH tunnels (russh) and SOCKS5/HTTP CONNECT proxies, all exposed as a local TCP forwarder so every driver works through them unchanged. |
| `dbayam-config` | Paths, settings, keymaps, connection store, secret stores, importers, query history (SQLite), projects/scripts/bookmarks/virtual model, SQL templates. |
| `dbayam-transfer` | Streaming exporters (CSV, JSON, SQL, XLSX, Markdown, HTML, XML, TXT, DBUnit, PHP, Parquet), importers (CSV, XLSX, XML, JSON), type inference, migration between databases, mock data. |
| `dbayam-compare` | Schema diff → dependency-ordered DDL, HTML report, Liquibase changelog, diagram diff; data compare with sync script. |
| `dbayam-tasks` | Task definitions, schedules (cron/interval/daily), scheduler, run log, OS scheduler integration (launchd/cron/schtasks). |
| `dbayam-ai` | Anthropic Messages API and OpenAI-compatible clients (streaming), prompts, schema context builder. |
| `dbayam-grid` | Virtual data grid (custom painted, both axes virtualised), result model with pending edits → DML, record/text/JSON/XML views, value/metadata/grouping/calc/references panels, virtual columns. |
| `dbayam-editor` | SQL editor widget (highlighting, autocomplete popup, diagnostics, hyperlinks, find/replace, editing commands). |
| `dbayam-visual` | ER diagrams (layered layout, edit mode, SVG/PNG export), plan diagrams, visual query builder, charts, dashboards, spatial viewer. |
| `dbayam-app` | The application: windows, tabs, navigator, execution engine, wiring of all crates, CLI modes, benchmarks. |

## Key design decisions

* **Driver contract.** `Session::execute` returns a `'static` cursor; at most one
  cursor is open per session. `SessionHandle` (app) enforces this by closing the
  open cursor before any other operation, so drivers that move the connection
  into the cursor (MySQL, SQLite) never deadlock.
* **Separate connection per editor** (default on, as in DBeaver): long queries in
  an editor never block the navigator. PostgreSQL databases other than the
  connected one get their own sessions on demand.
* **Bounded memory.** Result rows are fetched in pages (fetch size, default 200).
  `RowStore` keeps rows in pages and spills least-recently-used pages to an
  anonymous temp file when a per-store or global budget is exceeded; inactive
  result tabs release their pages. A 1M-row result therefore costs memory only
  for the visible/cached pages.
* **Text protocol for PostgreSQL results.** User queries are prepared (types,
  source table OIDs for editability) and streamed with the simple query
  protocol, so every type — including extension types — displays exactly as the
  server prints it.
* **Tunnels as local forwarders.** SSH and proxies listen on `127.0.0.1:<port>`;
  drivers connect there and verify TLS against the real host name
  (`ConnectParams::tls_server_name`).
* **Secrets** never touch disk in plain text: macOS Keychain by default, or an
  Argon2id + XChaCha20-Poly1305 vault protected by a master password. Config files
  are written atomically with mode 0600. `Secret` has no `Serialize` impl and
  redacts itself in `Debug`/`Display`.
* **Localisation.** English source strings are keys; `locales/id.tsv` holds the
  Indonesian translations (`dbayam_core::tr`).
* **Privacy.** No telemetry. The only outbound connections are the configured
  database servers, SSH/proxy hosts and — only when enabled — the AI provider.
