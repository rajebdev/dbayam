# DBayam

**A lightweight SQL database client, written entirely in Rust.**

DBayam is a native alternative to DBeaver for SQL databases on macOS, Windows and Linux: one small
binary, no JVM, no Electron.
It opens in about 0.1 s, idles at 0 % CPU and uses around 90 MB of memory — and result sets of
millions of rows never have to fit in memory.

> **Fork.** DBayam is a fork and an improved continuation of
> [DBiji](https://github.com/ilhamnadhif/dbiji) by Ilham Nadhif, which this project renamed. The fork
> lives at [rajebdev/dbayam](https://github.com/rajebdev/dbayam); it keeps the original MIT / Apache-2.0
> licence and copyright, and adds its own fixes, drivers and features (see the [changelog](CHANGELOG.md)).
> The upstream project is not affiliated with, nor endorses, this fork.

[Bahasa Indonesia](README.id.md)

![DBayam: navigator, SQL editor and result grid](docs/screenshots/editor.png)

## Highlights

- **Many databases, one interface**: PostgreSQL, CockroachDB, MySQL, MariaDB, SQLite, SQL Server,
  ClickHouse, MongoDB, Redis and (optional) DuckDB.
- **SQL editor** with schema-aware autocomplete (tables, columns, joins, aliases, CTEs), templates,
  `:name` / `${name}` parameters, `@set` / `@include` / `@export`, formatter, outline, error markers,
  explain plans (text, tree, diagram) and a searchable query history.
- **Data grid** that scrolls 1 M rows at under 1 ms per frame: inline editing saved in one
  transaction, foreign key navigation, server- or client-side filter and sort, record / JSON / XML
  views, value panels, charts and a map for geometry columns.
- **Safe by default**: development / test / production connection types, confirmations for
  `DELETE`/`UPDATE` without `WHERE` and `DROP`, read-only connections, a separate session per editor.
- **Connections** through SSH tunnels, SOCKS5 / HTTP proxies and SSL. Passwords live in the system's
  keychain (macOS Keychain, Windows Credential Manager, GNOME Keyring / KWallet) or an encrypted vault
  with a master password — never in a file. A connection dropped while the computer slept or the
  network was away connects again by itself.
- **Import** connections from DBeaver, `~/.pgpass` and URLs.
- **Everything else DBeaver users expect**: export / import (CSV, JSON, SQL, XLSX, XML, Parquet …),
  migration between databases, mock data, ER diagrams, visual query builder, schema and data compare
  (with Liquibase output), table designer, tasks and scheduler, backup / restore, session and lock
  manager, server dashboard and PostgreSQL debugger.
- **AI assistant** (optional, with your own API key: Anthropic, OpenAI or any compatible service) in a
  side panel: it knows the schema, writes and explains SQL, keeps its conversations and, if you
  allow it, runs read-only queries itself.
- English and Indonesian UI, light and dark themes (View → Theme or the sun / moon button in the status
  bar), customisable shortcuts and toolbar, no telemetry.
- **Automatic updates** from GitHub releases (optional), with one click to install, and release notes after every update.

<p>
  <img src="docs/screenshots/data.png" width="49%" alt="A table's data: grid with server-side filter and ORDER BY">
  <img src="docs/screenshots/er-diagram.png" width="49%" alt="ER diagram of the sample database">
</p>

## Install

Every [release](https://github.com/rajebdev/dbayam/releases/latest) is built for macOS, Windows and
Linux:

| System | Download |
| --- | --- |
| macOS 12 or later (Apple Silicon, Intel) | `DBayam-<version>.dmg`, or the [one-line installer](#macos) / [Homebrew](#homebrew) |
| Windows 10 / 11 (x64) | `DBayam-<version>-windows-x64-setup.exe` (installer) or `-windows-x64.zip` (portable) |
| Ubuntu 22.04+, Debian 12+, Linux Mint, Pop!_OS | `DBayam-<version>-linux-amd64.deb` (`arm64` on ARM) |
| Fedora, RHEL / Rocky / Alma 9+, openSUSE | `DBayam-<version>-linux-x86_64.rpm` (`aarch64` on ARM) |
| Arch Linux, Manjaro, EndeavourOS | `DBayam-<version>-linux-x86_64.pkg.tar.zst` (`aarch64` on ARM) |
| Any other Linux | `DBayam-<version>-linux-x86_64.AppImage` or `.tar.gz` (`aarch64` on ARM) |

### macOS

**One-line installer** (recommended):

```sh
curl -fsSL https://raw.githubusercontent.com/rajebdev/dbayam/master/scripts/install.sh | bash
```

The script downloads the latest release, checks it (the SHA-256 GitHub publishes, the code
signature, the bundle id and version), installs `DBayam.app` into `/Applications`, and opens it. Run
it again to update. To install a specific version, pass it as an argument: `… | bash -s -- v0.3.0`.

**Disk image**: download **`DBayam-<version>.dmg`**, open it and drag **DBayam** onto **Applications**.

> **“DBayam” Not Opened / “Apple could not verify…”?** Release builds are not notarized by Apple yet,
> so macOS blocks apps downloaded with a browser. Do one of these:
>
> - **Use the one-line installer** above. Nothing downloaded with `curl` is blocked.
> - **Allow it once in Settings**: click **Done** in the warning, open **System Settings → Privacy &
>   Security**, and click **Open Anyway** next to “DBayam was blocked…”.
> - **In Terminal**: `xattr -dr com.apple.quarantine /Applications/DBayam.app`

### Homebrew

From a clone of this repository (the cask follows every release):

```sh
brew install --cask ./packaging/homebrew/dbayam.rb
```

### Windows

Run **`DBayam-<version>-windows-x64-setup.exe`**. It installs DBayam for your user (no administrator
rights; pick “all users” in its first dialog to install it for everyone), adds it to the Start menu
and can be removed in **Settings → Apps**. To update, run the installer of the new version.

For a **portable** copy, unzip `DBayam-<version>-windows-x64.zip` and run `dbayam.exe` (with
`--portable <DIR>` to keep its settings and data in that folder).

> **“Windows protected your PC”?** The builds are not code-signed yet, so SmartScreen warns about a
> new download: click **More info → Run anyway**.

### Linux

Download the package for your distribution and install it:

```sh
# Ubuntu, Debian, Linux Mint, Pop!_OS
sudo apt install ./DBayam-<version>-linux-amd64.deb
# Fedora, RHEL / Rocky / Alma
sudo dnf install ./DBayam-<version>-linux-x86_64.rpm
# openSUSE
sudo zypper install --allow-unsigned-rpm ./DBayam-<version>-linux-x86_64.rpm
# Arch Linux, Manjaro, EndeavourOS
sudo pacman -U ./DBayam-<version>-linux-x86_64.pkg.tar.zst
```

On any other distribution, use the **AppImage** (`chmod +x DBayam-*.AppImage`, then run it) or unpack
the **tar.gz** into `~/.local` (the program, its menu entry and icon):
`tar -xzf DBayam-<version>-linux-x86_64.tar.gz -C ~/.local --strip-components=1`.

DBayam then is in the applications menu (or run `dbayam`). It needs glibc 2.34 or later (Ubuntu 22.04,
Debian 12, Fedora 35, RHEL 9 and newer), X11 or Wayland with OpenGL. Passwords are kept in the
desktop's keyring through the Secret Service (GNOME Keyring, KWallet) and file dialogs use the XDG
desktop portal — GNOME, KDE and most desktops have both; without a keyring, DBayam keeps passwords in
its vault with a master password. To update, install the package of the new version.

### From source

```sh
cargo build --release -p dbayam-app                     # target/release/dbayam (dbayam.exe)
cargo build --release -p dbayam-app --features duckdb   # with the embedded DuckDB engine
scripts/bundle-macos.sh                                # macOS: dist/DBayam.app, DBayam-<version>.dmg and .zip
scripts/bundle-linux.sh                                # Linux: .deb, .rpm, Arch package, AppImage and .tar.gz
pwsh scripts/bundle-windows.ps1                        # Windows: installer and .zip (needs Inno Setup 6)
```

## Getting started

1. **New connection** (or **Import connections from DBeaver**) on the welcome page — or click
   **Create sample database** to try everything on a small SQLite shop database.
2. Open a SQL editor with `⌘]`, type a query and press `⌘↩`.
3. Double-click a table in the navigator to edit its data; click a column header to sort it
   (ascending → descending → off); `⌘S` saves the changes.

## Updates

If you allow it, DBayam looks for a new release when it starts and every 6 hours. A new version
opens a dialog with the current and new version and what's new (found later, it lights up an
**Update** button in the toolbar instead of interrupting you). On Windows, **Download** gets the new
installer (run it over the installed version); on Linux it opens the release page with the packages.
On macOS, **Install update**:

1. downloads the new version and verifies it (size, the SHA-256 GitHub publishes, bundle id,
   version and code signature);
2. after **Restart now**, quits DBayam (asking about unsaved scripts first), swaps in the new app and
   opens it again; the previous copy is put back if anything goes wrong.

macOS asks for an administrator password only when your user cannot replace the app. Help → Check
for updates… checks right away; **Skip this version** and **Remind me later** are there too. After an
update DBayam shows what changed (also in Help → What's new; all notes are in
[CHANGELOG.md](CHANGELOG.md)). The one-line installer above also updates, and Homebrew users update
with `brew upgrade --cask dbayam`. The update check is one anonymous request to GitHub; nothing about
you or your databases is sent.

Installing in place needs DBayam in `/Applications` (or another folder it can write to); version
0.3.0 is the first with the updater.

## Command line

```text
dbayam [FILE.sql | DATABASE.db ...]   open scripts, SQLite / DuckDB files
dbayam --execute FILE.db FILE.sql     open and run the script on the database
dbayam --run-task <id|name>           run a saved task without a window (exit code 0/1/2/3)
dbayam --portable <DIR>               keep all settings and data in DIR
dbayam --bench all                    startup, memory, grid and editor benchmarks
```

## Keyboard shortcuts

| Action | Shortcut |
| --- | --- |
| Execute statement / selection | `⌘↩` |
| Execute script | `⌥X` |
| Execute in a new result tab | `⇧⌘↩` |
| Explain plan | `⇧⌘E` |
| Cancel query | `⌘.` |
| New SQL editor | `⌘]` |
| Active connection / schema | `⌘9` / `⌘0` |
| Format SQL | `⇧⌘F` |
| Autocomplete | `⌃Space` |
| Command palette | `⇧⌘P` |
| Save script / data | `⌘S` |

On Windows and Linux, `⌘` is `Ctrl`, `⌥` is `Alt` and `⌃` is `Ctrl` as well. Every shortcut can be
changed in **Settings → Keyboard shortcuts** (DBayam or DBeaver preset).

## Performance

Measured on the release build (Apple M1 Pro) with `dbayam --bench all`:

| | Target | Measured |
| --- | --- | --- |
| Binary size (macOS) | < 40 MB | 39.9 MB |
| Cold start to first frame | < 1 s | ≈ 0.12 s |
| Idle memory | < 120 MB | ≈ 93 MB |
| 3 connections + 5 result tabs | < 250 MB | ≈ 93 MB |
| 1 M rows scroll | < 16 ms / frame | p95 0.5 ms |
| Typing in a 50 000-line script | < 16 ms / frame | p95 0.5 ms |

## Security and data

- No telemetry. DBayam only connects to the servers you configure and, if you enable them, to your AI
  provider and to GitHub for the update check (one anonymous request for the latest release when
  DBayam starts and every 6 hours; asked once, changeable in Settings → General).
- Passwords and API keys are stored in the system's credential store (macOS Keychain, Windows
  Credential Manager, the Secret Service on Linux) or in `vault.bin` (Argon2id +
  XChaCha20-Poly1305); configuration files are written with mode 0600.
- Settings, connections, scripts, history and tasks live in
  `~/Library/Application Support/dev.dbayam.DBayam/` on macOS, `%APPDATA%\dbayam\DBayam\` on Windows and
  `~/.config/dbayam/` + `~/.local/share/dbayam/` on Linux (or `DBAYAM_HOME` / `--portable`).

## Building and testing

```sh
cargo test --workspace --exclude dbayam-driver-duckdb
# integration tests against real servers (Docker):
export DBAYAM_TEST_PG=postgres://dbayam:dbayam@127.0.0.1:55432/dbayam
export DBAYAM_TEST_MYSQL=mysql://root:dbayam@127.0.0.1:53306/dbayam
export DBAYAM_TEST_MARIADB=mariadb://root:dbayam@127.0.0.1:53307/dbayam
export DBAYAM_TEST_COCKROACH='cockroachdb://root@127.0.0.1:26257/defaultdb?sslmode=disable'
export DBAYAM_TEST_MONGO=127.0.0.1:27018   # mongod on that port
export DBAYAM_TEST_REDIS=127.0.0.1:6390   # redis-server on that port
cargo test -p dbayam-app
```

Signing and notarization: set `DBAYAM_SIGN_IDENTITY` (Developer ID) and `DBAYAM_NOTARY_PROFILE`
before running `scripts/bundle-macos.sh`.

## Project layout

| Crate | Role |
| --- | --- |
| `dbayam-app` | The application: window, tabs, navigator, execution engine, CLI |
| `dbayam-core` | Shared contract: values, driver/session traits, metadata, DDL, dialects, row store, i18n |
| `dbayam-sql` | Tokenizer, splitter, formatter, autocomplete, analysis, parameters |
| `dbayam-driver-*` | PostgreSQL, MySQL, SQLite, SQL Server, ClickHouse, DuckDB, MongoDB, Redis drivers |
| `dbayam-editor` / `dbayam-grid` / `dbayam-visual` | Editor widget, data grid, diagrams and charts |
| `dbayam-config` / `dbayam-net` | Settings, secrets, importers / SSH tunnels and proxies |
| `dbayam-transfer` / `dbayam-compare` / `dbayam-tasks` / `dbayam-ai` | Data transfer, compare, tasks, AI |

More in [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) and the DBeaver parity matrix in
[docs/PARITY.md](docs/PARITY.md).

## Known limitations

- Oracle, Db2, Firebird, H2 and Trino are not supported (no pure-Rust network driver yet).
- MongoDB and Redis have no SQL: their editors take the shell language (`db.users.find({…})`) and
  Redis commands (`HGETALL session:9`), one statement per line or `;`. Cell editing, SQL tools
  (query builder, ER diagram, table designer, data search) and transactions do not apply to them.
- Kerberos and MySQL multi-factor authentication are not supported.
- Release builds are not notarized (macOS) or code-signed (Windows) yet.

## License

MIT OR Apache-2.0.

The bundled Inter typeface (`crates/dbayam-app/assets/fonts`) is licensed under the SIL Open Font License 1.1 (`Inter-LICENSE.txt`).
