# CLAUDE.md - Guidelines for Working with this Codebase

## Commands
- 🏗️ Run dev server: `cargo loco start` or `cargo loco start --server-and-worker`
- 🧪 Run all tests: `cargo test`
- 🧪 Run specific test: `cargo test module::submodule::test_name` (e.g. `cargo test models::users::test_can_validate_model`)
- 🧪 Show test output: `cargo test test_name -- --nocapture`
- 🧹 Format code: `cargo fmt --all`
- 🔍 Lint (strict): `cargo clippy --all-features -- -D warnings -W clippy::pedantic -W clippy::nursery -W rust-2018-idioms`
- 🔍 Lint (2024 compat): `cargo clippy --all-features -- -D warnings -W clippy::pedantic -W clippy::nursery -W rust-2024-compatibility`

## Code Style
- 🧩 Imports: group by external crates first, then internal modules
- 🏷️ Types: use proper Rust types; prefer strong typing with custom structs
- 📝 Error handling: use `ModelResult<T>` for model operations with proper error propagation
- 🔤 Naming: snake_case for functions/variables, CamelCase for types/traits
- 📚 Documentation: document public functions with triple-slash /// comments, especially error cases
- 🧠 Use `#[must_use]` for functions returning important values
- 🔄 Favor `async/await` patterns for database operations
- 📏 Format with cargo fmt and follow Rust 2024 compatibility guidelines