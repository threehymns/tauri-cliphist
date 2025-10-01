# Tauri Cliphist

A cross-platform desktop application that provides a modern GUI interface for the [cliphist](https://github.com/sentriz/cliphist) clipboard manager. Built with Tauri and Svelte, it offers efficient clipboard history management with search, copy, and delete functionality.

## Features

- **Clipboard History**: View your persistent clipboard history stored by cliphist
- **Fast Search**: Efficient fuzzy search through clipboard entries
- **Copy to Clipboard**: One-click copying of any historical entry back to clipboard
- **Delete Entries**: Remove unwanted entries from history
- **Virtual Scrolling**: High-performance rendering for large histories
- **Dark/Light Theme**: Automatic theme support
- **Keyboard Shortcuts**: Press ESC to close the application
- **Cross-Platform**: Works on Linux (Wayland/X11), Windows, and macOS. (Note however that cliphist itself is only available on Linux)

## Prerequisites

Before running this application, you need to install the following dependencies:

### Cliphist
The core clipboard manager that stores your clipboard history persistently.

**Installation:**
```bash
# On Arch Linux
sudo pacman -S cliphist

# On other distributions, build from source:
git clone https://github.com/sentriz/cliphist.git
cd cliphist
go build
sudo mv cliphist /usr/local/bin/
```

### Clipboard Tools
For copying content back to clipboard:

**Wayland (recommended):**
```bash
# Arch Linux
sudo pacman -S wl-clipboard

# Ubuntu/Debian
sudo apt install wl-clipboard
```

**X11 (fallback):**
```bash
# Arch Linux
sudo pacman -S xclip

# Ubuntu/Debian
sudo apt install xclip
```

## Installation

### Download Pre-built Binary
Download the latest release from the [releases page](https://github.com/threeehymns/tauri-cliphist/releases).

### Build from Source
1. Clone the repository:
```bash
git clone https://github.com/threeehymns/tauri-cliphist.git
cd tauri-cliphist
```

2. Install frontend dependencies:
```bash
bun install
```

3. Build the application:
```bash
bun tauri build
```

The built application will be available in `src-tauri/target/release/`.

## Development

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable)
- [Bun](https://bun.sh/) or Node.js
- Tauri CLI: `cargo install tauri-cli`

### Setup
```bash
# Install dependencies
bun install

# Start development server
bun run tauri dev
```

### Available Scripts
- `bun run dev` - Start the Vite development server
- `bun run build` - Build the frontend for production
- `bun run check` - Run TypeScript checks
- `bun run tauri dev` - Start Tauri development mode
- `bun run tauri build` - Build the application

### Project Structure
```
src/
├── routes/
│   ├── +layout.svelte    # App layout
│   ├── +layout.ts        # SvelteKit config
│   └── +page.svelte      # Main page
├── app.css               # Global styles
└── app.html              # HTML template

src-tauri/
├── src/
│   ├── main.rs           # Application entry point
│   └── lib.rs            # Tauri commands and logic
├── Cargo.toml            # Rust dependencies
└── tauri.conf.json       # Tauri configuration
```

## Architecture

### Backend (Rust/Tauri)
- Executes `cliphist` commands to interact with clipboard history
- Provides Tauri commands for frontend interaction
- Implements fuzzy search algorithm
- Handles clipboard operations via system tools

### Frontend (Svelte)
- Modern UI with Svelte 5 runes
- Virtual scrolling for performance with large histories
- Responsive design with Tailwind CSS
- Dark/light theme support

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Make your changes and test thoroughly
4. Run `bun run check` to ensure TypeScript compliance
5. Commit your changes: `git commit -am 'Add some feature'`
6. Push to the branch: `git push origin feature/your-feature`
7. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [cliphist](https://github.com/sentriz/cliphist) - The underlying clipboard manager
- [Tauri](https://tauri.app/) - Cross-platform desktop app framework
- [Svelte](https://svelte.dev/) - Frontend framework
- [TanStack Virtual](https://tanstack.com/virtual/) - Virtual scrolling library