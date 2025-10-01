# Agent Instructions for tauri-cliphist

## Build/Test Commands
- **Frontend build**: `bun run build`
- **Frontend dev**: `bun run dev`
- **TypeScript check**: `bun run check`
- **Rust build**: `cargo build`
- **Rust test**: `cargo test`
- **Rust check**: `cargo check`
- **Tauri dev**: `bun run tauri dev`
- **Tauri build**: `bun run tauri build`

## Project Overview
This is a Tauri application that provides a GUI frontend for the cliphist clipboard manager. It integrates with the cliphist command-line tool to store, retrieve, and manage clipboard history persistently.

## Key Dependencies
- **cliphist**: External clipboard history manager (must be installed separately)
- **wl-clipboard/xclip**: System clipboard access tools
- **Tauri**: Cross-platform desktop app framework
- **Svelte 5**: Frontend framework with runes
- **@tanstack/svelte-virtual**: High-performance list virtualization

## Code Style Guidelines

### TypeScript/Svelte
- Use strict TypeScript with explicit types
- Use Svelte 5 runes (`$state`, `$derived`, etc.)
- Interface definitions for data structures
- camelCase for variables/functions, PascalCase for types
- ES6 imports with named imports preferred
- Async functions for Tauri invoke calls
- Use TanStack Virtual for large lists: `createVirtualizer()` with reactive count

### Rust
- Execute external commands using `std::process::Command`
- Use `Result<T, E>` for error handling with custom error types
- Parse command output manually (no external parsing libraries)
- Implement efficient fuzzy search combining substring and word-based matching
- snake_case for functions/variables, PascalCase for structs/enums
- Group imports: external crates → std → local modules

### General
- No specific linter/formatter configured - follow language conventions
- Use meaningful variable names
- Keep functions focused and single-purpose
- Handle external command failures gracefully
- Add comments only when code intent isn't clear