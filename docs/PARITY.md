# DBeaver parity matrix

Every row of the PRD §4 inventory, with where it is implemented and the
automated test that covers it, or the recorded reason for an exception
(PRD §13, phase 3 gate). Test names are `crate::file::test` (integration
tests in `crates/<crate>/tests/`). Tests marked *(env)* need the Docker
servers from `.github/workflows/ci.yml` (`DBAYAM_TEST_PG`, `DBAYAM_TEST_MYSQL`,
`DBAYAM_TEST_MARIADB`, `DBAYAM_TEST_CLICKHOUSE`, `DBAYAM_TEST_MSSQL`).

Legend: ✅ implemented and tested · 🟡 implemented, partially tested ·
⛔ exception (reason given).

## Connections

| Feature | Status | Where | Tests |
| --- | --- | --- | --- |
| Connection wizard (all driver properties, URL paste, test) | ✅ | `dbayam-app/src/dialogs/connection.rs` | `dbayam-app::tabs_smoke`, `dbayam-config::import::url` unit tests |
| Connection types dev/test/prod, colours, confirmations, auto-commit | ✅ | `dbayam-config::connections`, settings → Connection types | `dbayam-app::editor_e2e::editor_executes_…` (confirmations) |
| SSH tunnel (password, keys, agent; host key policies) | ✅ | `dbayam-net::tunnel`, `ssh` | `dbayam-net::tunnel` (17 end-to-end tests against an in-process SSH server) |
| SSL with truststore / client certificates | ✅ | drivers (`ConnectParams.ssl`) | `dbayam-driver-postgres::integration` TLS modes *(env)*, `dbayam-driver-mysql::connect` *(env)* |
| Proxy (SOCKS5, HTTP CONNECT) | ✅ | `dbayam-net::socks`, `http_proxy` | `dbayam-net::tunnel` proxy → SSH chains |
| Network profiles | ✅ | `ConnectionStore::network_profiles`, settings → Network profiles, connection dialog selector | `dbayam-config::connections` unit tests |
| Init SQL and shell commands before/after connect | ✅ | `dbayam-app::conn::connect_task`, `dbayam-net::process` | `dbayam-net::process` unit tests |
| Auto-commit / manual mode, pending transaction, transaction log | ✅ | `SessionHandle::tx_state`, SQL editor toolbar | `dbayam-app::data_edit_e2e` (save inside transactions) *(env)* |
| Separate connection per editor | ✅ | `InitConfig::separate_editor_connection`, `SqlEditorTab::session` | `dbayam-app::editor_e2e` |
| Driver manager | ⛔ | Drivers are compiled in (no JDBC jars to manage); *Help → Database drivers* lists them with capabilities | — |
| Import connections (DBeaver, pgpass, URL) | ✅ | `dbayam-config::import`, `dialogs/import.rs` | `dbayam-config::tests::dbeaver_import` (no lost fields), url/pgpass unit tests |

## Authentication and security

| Feature | Status | Where | Tests |
| --- | --- | --- | --- |
| User / password | ✅ | all drivers | every integration suite *(env)* |
| pgpass | ✅ | `AuthMethod::Pgpass`, `import::pgpass::lookup` | `dbayam-config::import::pgpass` unit tests |
| Password from shell command | ✅ | `AuthMethod::Command`, `dbayam-net::password_from_command` | `dbayam-net::process` unit tests |
| Kerberos | ⛔ | No maintained pure-Rust GSSAPI implementation; would need the system Kerberos C libraries, which the PRD's "full Rust" rule excludes for network clients | — |
| MySQL 2FA (multi-factor) | ⛔ | Not supported by `mysql_async`; tracked for when the protocol support lands upstream | — |
| Secret storage with master password | ✅ | `dbayam-config::secrets` (Keychain, Argon2id + XChaCha20-Poly1305 vault) | `dbayam-config::secrets` unit tests (vault, wrong password, migration) |
| Read-only connections | ✅ | client-side classification (`dbayam-sql::classify`) | `dbayam-app::editor_e2e::read_only_connections_block_writes` |
| Change database password | ✅ | `tabs/dbtools.rs` (`ChangePasswordTab`) | see *Administration* |

## Navigator

| Feature | Status | Where | Tests |
| --- | --- | --- | --- |
| Connection / object tree, lazy and cached | ✅ | `nav.rs`, `conn/meta.rs` | `dbayam-app::app_e2e::execute_fetch_count_and_metadata` *(env)* |
| Simple and advanced views | ✅ | `NavigatorView` per connection | `dbayam-app::tabs_smoke` |
| Object filters | ✅ | `ObjectFilters`, connection dialog → Filters | `dbayam-core` filter unit tests |
| Bookmarks | ✅ | `Project::add_bookmark`, Project panel | `dbayam-config::project` unit tests |
| Projects and project explorer (scripts, diagrams) | ✅ | `dbayam-config::project`, `project_view.rs` | `dbayam-config::project` unit tests |
| Metadata search | ✅ | `Session::search_metadata`, `tabs/search.rs` | driver suites *(env)* |
| Full-text / data search | ✅ | `tabs/search.rs` (data search over text columns) | `dbayam-app::tabs_smoke` |
| Properties editor | ✅ | object editor → Properties (+ virtual model) | `dbayam-app::tabs_smoke` |

## SQL editor

| Feature | Status | Where | Tests |
| --- | --- | --- | --- |
| Many scripts per connection saved as files, restored on start | ✅ | `SqlEditorTab::save`, `ProjectManager::{load,save}_session` | `dbayam-config::project` unit tests |
| Execute statement / script / selection, new result tab, cancel | ✅ | `sqlprep.rs`, `exec.rs` | `dbayam-app::editor_e2e`, `dbayam-app::app_e2e::errors_stop_the_script_and_cancel_works` *(env)* |
| Results in several tabs or one | ✅ | result tabs, *Open each query in a new tab* | `dbayam-app::editor_e2e` |
| Autocomplete and semantic analysis (tables, columns, joins, aliases, CTEs) | ✅ | `dbayam-sql::complete`, `sqllang.rs` | `dbayam-sql::complete` 42 scenarios on the standard schema |
| Templates | ✅ | `TemplateLibrary`, offered in autocomplete | `dbayam-app::sqllang::tests` |
| Formatter | ✅ | `dbayam-sql::format` | formatter corpus (PostgreSQL, MySQL, SQLite, T-SQL × 5 option sets, idempotent) |
| Variables / parameters panel | ✅ | parameter prompt + *Variables* result tab | `dbayam-app::sqlprep::tests`, `editor_e2e` |
| Object hyperlinks | ✅ | `EditorLanguage::link_at` | `dbayam-editor` headless tests |
| Outline | ✅ | `dbayam-sql::outline`, editor outline panel | `dbayam-sql::outline` tests |
| Error indicators | ✅ | server error position → editor diagnostic; unknown table/column diagnostics | `dbayam-app::editor_e2e` |
| Spell checker | ✅ | `dbayam-sql::spell` (English dictionary + user words) | `dbayam-sql::spell` tests |
| SQL generation (SELECT/INSERT/UPDATE/DELETE/UPSERT/DDL) | ✅ | `dbayam-sql::generate`, navigator → Generate SQL | `dbayam-sql::generate` tests |
| SQL terminal | ✅ | editor *Terminal mode*, SQL console tabs | `dbayam-app::tabs_smoke` |
| Client commands `@set`, `@include`, `@export`, `@echo` | ✅ | `dbayam-sql::commands`, `sqlprep.rs` | `dbayam-app::sqlprep::tests`, `editor_e2e` |
| Query manager (history) | ✅ | `dbayam-config::history`, `tabs/history.rs` | `dbayam-config::history` tests |
| Switch active connection / schema | ✅ | editor toolbar, ⌘9 / ⌘0 | `dbayam-app::keys::tests` |
| Execute with the native client | ✅ | `Driver::client_command` + `tools::run_command` | driver command tests |

## Query plan and builder

| Feature | Status | Where | Tests |
| --- | --- | --- | --- |
| Execution plan (text, tree, diagram; EXPLAIN ANALYZE) | ✅ | drivers' `explain`, `dbayam-visual::plan` | `editor_e2e` (explain), driver explain tests *(env)*, `dbayam-visual` plan tests |
| Visual query builder | ✅ | `dbayam-visual::builder`, `tabs/query_builder.rs` | `dbayam-visual` builder tests (SQL generation) |

## Data editor

| Feature | Status | Where | Tests |
| --- | --- | --- | --- |
| Grid, text, record, JSON, XML views; spatial | ✅ | `dbayam-grid`, `dbayam-visual::spatial` | `dbayam-grid` UI tests, spatial parser tests |
| Server- or client-side filter and sort | ✅ | `ResultPanel::requery`, grid client views | `dbayam-grid::view` tests |
| Inline editing, add / duplicate / delete rows, save / cancel | ✅ | `ResultModel`, `SessionHandle::apply_changes` | `dbayam-app::data_edit_e2e::edit_save_and_cancel` (PostgreSQL, MySQL, MariaDB, SQLite) |
| Value, references, metadata, grouping, calc panels | ✅ | `dbayam-grid::panels` | `dbayam-grid` UI tests |
| Advanced copy (TSV, CSV, Markdown, JSON, SQL, HTML, XML) | ✅ | `dbayam-grid::copy` | `dbayam-grid::copy` tests |
| Foreign key navigation with history | ✅ | `ResultPanel::navigate` | `dbayam-app::tabs_smoke` |
| Refresh and auto-refresh, fetch size, total count | ✅ | `ResultSource` | `dbayam-app::app_e2e::execute_fetch_count_and_metadata` |
| Virtual columns and virtual keys | ✅ | object editor → Virtual model, `ResultPanel` | `dbayam-app::data_edit_e2e::virtual_model_dictionary_and_virtual_columns` |
| Data dictionary | ✅ | dictionary description columns | same test |
| Data formats | ✅ | settings → Result sets → Value formats | `dbayam-core::value` tests |

## Charts and special data

| Feature | Status | Where | Tests |
| --- | --- | --- | --- |
| Charts from results (line, bar, area, scatter, pie) | ✅ | `dbayam-visual::chart`, result panel *Chart* | `dbayam-visual` chart tests |
| Dashboards (server health) | ✅ | `tabs/admin.rs` (`DashboardTab`), `dbayam-visual::dashboard` | `dbayam-visual` dashboard tests |
| Mock data generation | ✅ | `dbayam-transfer::mock`, `tabs/transfer` | `dbayam-transfer::mock` tests |
| GIS / spatial data | ✅ | `dbayam-visual::spatial` (WKT, WKB, EWKB, MySQL internal) | spatial tests |
| XML / JSON data | ✅ | grid JSON/XML views and value panel | `dbayam-grid` view tests |

## ER diagrams

| Feature | Status | Where | Tests |
| --- | --- | --- | --- |
| Automatic diagrams from a schema or objects | ✅ | `dbayam-visual::er`, `tabs/er.rs` | layout tests; 200-table benchmark (≤ 1.7 ms/frame) |
| Custom saved diagrams, image export | ✅ | project diagrams, SVG/PNG export | `dbayam-visual::er` export tests |
| Edit mode (change the schema from the diagram) | ✅ | edit events → reviewed DDL | `dbayam-visual` UI tests |

## Data transfer

| Feature | Status | Where | Tests |
| --- | --- | --- | --- |
| Export: CSV, DBUnit, JSON, Markdown, PHP, SQL, TXT, XML, XLSX, HTML (+ Parquet, JSON Lines, TSV) | ✅ | `dbayam-transfer::export`, `tabs/transfer` | `dbayam-transfer::formats`, `roundtrip` |
| Import: CSV, XLSX, XML (+ JSON) | ✅ | `dbayam-transfer::import` | `dbayam-transfer::import`, `roundtrip` |
| Import and replace | ✅ | `ImportMapping::mode` | `dbayam-transfer::import` |
| Migration between databases | ✅ | `dbayam-transfer::migration` | `dbayam-transfer::migration` |
| Save as task | ✅ | `TaskKind::{Export, Import, Migration}`, `transferexec.rs` | `dbayam-app::transfer_e2e` *(env)* |
| Round trip CSV / JSON / SQL produces identical data (PRD gate) | ✅ | — | `dbayam-transfer::roundtrip` |

## Comparison

| Feature | Status | Where | Tests |
| --- | --- | --- | --- |
| Schema compare → DDL script | ✅ | `dbayam-compare`, `tabs/compare.rs` | `dbayam-app::app_e2e::schema_compare_gate` (DDL makes schemas identical; PostgreSQL, MySQL, MariaDB) *(env)* |
| HTML report, diagram diff, Liquibase changelog | ✅ | `html_report`, *Diff diagram*, `liquibase_changelog` | `dbayam-compare` tests |
| Data compare with sync script | ✅ | `run_data_compare`, `tabs/data_compare.rs` | `dbayam-compare` data tests, `dbayam-app::tabs_smoke` |

## Tasks

| Feature | Status | Where | Tests |
| --- | --- | --- | --- |
| Task management, SQL script tasks, composite tasks | ✅ | `dbayam-tasks`, `tabs/tasks.rs`, `taskexec.rs` | `dbayam-tasks` tests, `dbayam-app::headless_task`, `dbayam-tasks` tests |
| Scheduler (in app, and launchd / cron / schtasks when closed) | ✅ | `Scheduler`, `dbayam_tasks::os`, `dbayam --run-task` | `dbayam-tasks` scheduler tests (DST, catch-up, cancellation) |

## Administration

| Feature | Status | Where | Tests |
| --- | --- | --- | --- |
| Backup and restore with the vendor tools | ✅ | `Driver::backup_command`/`restore_command`, `tools.rs`, `tabs/dbtools.rs` | `dbayam-app::backup_e2e` (PostgreSQL custom + plain; MySQL / MariaDB when the client tools are installed) *(env)* |
| Session manager, lock manager | ✅ | `tabs/admin.rs` | driver admin tests *(env)* |
| Maintenance tools | ✅ | `Dialect::maintenance_ops`, `tabs/dbtools.rs` | `dbayam-app::tabs_smoke` |
| PostgreSQL debugger | ✅ | `PgSession::debug_routine` (pldebugger), `DebuggerTab` | `dbayam-driver-postgres` debugger test *(env, pldebugger)* |
| Server health dashboard | ✅ | `DashboardTab` | `dbayam-app::tabs_smoke` |

## Schema and DDL

| Feature | Status | Where | Tests |
| --- | --- | --- | --- |
| Create tables, columns, indexes, constraints, foreign keys, triggers | ✅ | `tabs/table_designer.rs`, `DdlGenerator` | `dbayam-app::tabs_smoke`, driver DDL round trips |
| Partitions | 🟡 | shown in the object editor; created via SQL (the designer does not edit partition schemes) | driver metadata tests |
| DDL generation | ✅ | `Session::object_ddl`, `DdlGenerator` | driver DDL round-trip tests |
| Sample database | ✅ | *Help → Create sample database* (`sample.rs`) | `dbayam-app::sample::tests` |

## Interface

| Feature | Status | Where | Tests |
| --- | --- | --- | --- |
| Themes (light, dark, system), high contrast | ✅ | `theme.rs` | `dbayam-app::tabs_smoke` |
| Interface language (English, Indonesian) | ✅ | `dbayam-core::i18n`, `locales/id.tsv` | `dbayam-app::i18n_coverage`, `i18n_labels` |
| Keyboard shortcuts (customisable, DBayam / DBeaver presets) | ✅ | `dbayam-config::keymap`, `keys.rs` | `dbayam-config::keymap` tests, `dbayam-app::keys::tests` |
| Toolbar customisation | ✅ | settings → Toolbar | `dbayam-config::settings` tests |
| Accessibility | 🟡 | AccessKit tree (egui), editor text runs, keyboard navigation everywhere; screen reader support per grid cell is not implemented (one node per grid) | `dbayam-editor` AccessKit test |

## AI assistant

| Feature | Status | Where | Tests |
| --- | --- | --- | --- |
| Chat, query suggestions, explanations, error fixes, object descriptions | ✅ | `dbayam-ai`, `tabs/ai.rs` | `dbayam-ai` tests (mock HTTP server), `dbayam-app::tabs_smoke` (prompt assembly) |
| Several model providers | ✅ | Anthropic, OpenAI-compatible servers | `dbayam-ai` tests |

## Databases (PRD §7)

| Database | Status | Tests |
| --- | --- | --- |
| PostgreSQL | ✅ | `dbayam-driver-postgres` (66 unit, 23 integration) *(env)* |
| MySQL, MariaDB | ✅ | `dbayam-driver-mysql` (67 unit, 34 integration × 2 servers) *(env)* |
| SQLite | ✅ | `dbayam-driver-sqlite` (87) |
| SQL Server | ✅ | `dbayam-driver-mssql` (43 unit, 6 integration) *(env)*. 2012 and newer over TLS 1.2+; 2008 and older offer TLS 1.0 only, which rustls does not implement — connect with SSL `disable` when the server does not force encryption, otherwise the server needs an upgrade |
| ClickHouse | ✅ | `dbayam-driver-clickhouse` (54 unit, 5 integration) *(env)* |
| DuckDB | ✅ | `dbayam-driver-duckdb` (43), optional `duckdb` feature |
| MongoDB | ✅ | `dbayam-driver-mongodb` (26 unit): shell language (`db.c.find({…})`), documents as rows, collections / indexes / sampled fields, explain, `mongodb+srv://` seed lists, TLS `prefer` tries both modes, `authSource` follows the connection's database. Supports MongoDB 4.2–8.0 (`mongodb` 3.8); older servers are refused with the version they run *(live tests: `DBAYAM_TEST_MONGO`, `DBAYAM_TEST_MONGO_AUTH`)* |
| Redis | ✅ | `dbayam-driver-redis` (15): commands one per line, replies as tables, keys as tables, databases as schemas, session manager and dashboard *(no server in CI)* |
| CockroachDB | ✅ | PostgreSQL driver with CockroachDB catalog adjustments; `dbayam-app` end-to-end suites (execution, cancel, schema compare gate, data editing, editor) run against CockroachDB *(env `DBAYAM_TEST_COCKROACH`)* |
| Oracle | ⛔ | Only driver needs Oracle's native client library (OCI), excluded by the full-Rust rule (PRD §7) |
| Db2, Firebird, H2, Trino | ⛔ | No usable pure-Rust network driver yet (PRD §7: "only if a viable Rust driver exists") |

## Performance targets (PRD §2)

Measured with `dbayam --bench all` on the release build (macOS, Apple M1 Pro):

| Target | Result |
| --- | --- |
| Binary < 48 MB | 44.1 MiB (without DuckDB); 37.0 MiB before the MongoDB and Redis drivers, which the budget grew for |
| Cold start < 1 s | 105 ms to the first frame |
| Idle memory < 120 MB | 69 MB |
| 3 connections + 5 result tabs < 250 MB | 83 MB (20 000 rows fetched per tab) |
| 1 M rows scroll, < 16 ms/frame | p95 0.5 ms (grid benchmark), 1 M × 100 columns: avg < 3 ms, worst 11.7 ms |
| Editor typing at 50 000 lines | p95 0.4 ms |
