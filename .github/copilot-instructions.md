# Copilot Instructions

## Build, test, and lint commands

- Build library crate:
  - `cargo build`
- CI-equivalent workspace checks:
  - `cargo check --workspace --all-targets`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace --all-targets`
- Run tests for only the main library crate:
  - `cargo test -p tidlers`
- Run a single test/doctest by filter:
  - `cargo test -p tidlers <test_name_substring>`
- Run examples:
  - `cargo run -p testing-client`
  - `cargo run -p hires-streamer`
  - `cargo run -p login-save`

If workspace checks fail on Linux audio dependencies, CI installs `libasound2-dev` before running checks (`.woodpecker.yml`).

## High-level architecture

- Crate entrypoint is `src/lib.rs`, which exposes modules and re-exports `TidalClient`, `TidalError`, and `TidalSession`.
- `TidalClient` (`src/client/mod.rs`) is the main facade; it owns:
  - `session: TidalSession` (auth + locale/time/audio settings)
  - `rq: RequestClient` for HTTP transport
- API methods are split by domain in `src/client/api/*.rs` and implemented as `impl TidalClient` extension blocks.
- Request flow is layered:
  1. Endpoint method builds a request via `self.request(...)`
  2. `ApiRequestBuilder` (`src/client/api/request_builder.rs`) injects query params/headers/base URL and deserializes (`send`) or returns raw text (`send_raw`)
  3. `RequestClient` (`src/requests.rs`) executes `reqwest` calls and maps transport/status errors
- Authentication is split across:
  - `src/auth/init.rs` for `TidalAuth` construction and client-credential token flow
  - `src/client/oauth.rs` for OAuth device-code login and polling
  - `src/client/auth.rs` for refresh-token logic
- Payload types are in `src/client/models/**`, including auth/generic wrappers in `src/client/models/responses.rs`.
- Session persistence is implemented in `src/client/data.rs` via `get_json()` and `from_json()`.
- Typed ID wrappers (`TrackId`, `AlbumId`, etc.) are defined in `src/ids.rs` and accepted by endpoint methods via `impl Into<...Id>`.

## Key conventions

- Add new public API operations as `impl TidalClient` methods under the closest domain file in `src/client/api/`.
- Prefer `ApiRequestBuilder` (`.send()`) for standard JSON endpoints; use `.send_raw()` only for endpoints that need custom decoding/parsing (e.g., playback manifest handling in `track.rs`).
- Use the correct base URL constant per endpoint (`API_V1_LOCATION`, `API_V2_LOCATION`, `OPEN_API_V2_LOCATION`, `WEB_API_V2_LOCATION`); default transport base is API v1.
- Many endpoints call `.with_country_code()`, which depends on `self.user_info.country_code`; after OAuth/session restore, call `refresh_user_info()` before country-scoped requests.
- Keep internal plumbing internal: request-builder and low-level request constructors are `pub(crate)`; external usage should stay on `TidalClient` + model/config types.
- Preserve serde field mapping style used across models (`#[serde(rename = ...)]`, `#[serde(rename_all = "camelCase")]`) because API payload shapes vary by endpoint.
- Model naming convention:
  - Use `*Response` only for top-level endpoint return payloads.
  - Keep nested/resource/domain structs suffix-free (no `Response` postfix).
  - Do not use `*Page`; prefer `*Response` for paginated top-level payloads.
- Keep `CHANGELOG.md` current for significant user-facing changes; add/update an `Unreleased` entry when behavior or API surface changes in a meaningful way.
- Be explicit about the two search type enums:
  - Request config enum: `client::models::search::config::SearchType`
  - Parsed-result enum: `client::models::search::SearchType`
- `ApiRequestBuilder::send()` parse errors are expected to include response URL/status/body context; keep this behavior when touching shared request parsing.
