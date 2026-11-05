# NoteManager

A lightweight, terminal-based task and note management application built in Rust. Features automatic JSON persistence, intelligent ID recycling, in-place state updates, and an interactive Terminal User Interface (TUI).

---

## Features

- **Decoupled Architecture:** Clean separation of concerns between core business logic ([`src/lib.rs`](src/lib.rs)) and the frontend interface ([`src/main.rs`](src/main.rs)).
- **Interactive TUI:** Real-time navigation, task status toggling, and in-app task creation powered by `ratatui` and `crossterm`.
- **Automatic JSON Persistence:** Changes are serialized to and loaded from standard OS configuration directories (`~/.config/notemanager/tasks.json` on Linux or `~/Library/Application Support/notemanager/tasks.json` on macOS) via `dirs`, `serde`, and `serde_json`. Zero home-directory clutter.
- **Intelligent ID Recycling:** Automatically finds and assigns the lowest available unique identifier when new tasks are added.
- **Custom Parsing Engine:** Robust input validation and pattern matching supporting composite actions and multiple target IDs.

---

## Architecture & Concepts

This project showcases core Rust language fundamentals and idioms:

- **Ownership & Borrowing:** Clean handling of `&self` (read-only borrowing) vs. `&mut self` (exclusive mutable access) across state transitions.
- **Iterator Logic & Closures:** Efficient in-place filtering (`.retain()`) and non-allocating lookups (`.any()`).
- **Error Handling:** Idiomatic use of `Result<T, E>`, pattern matching, and structured error reporting.
- **Data Modeling:** Strongly typed domain models using `struct` and `enum` with derive macros (`Serialize`, `Deserialize`).

---

## Controls

| Key | Action |
| :--- | :--- |
| `↑` / `k` | Navigate to the previous task |
| `↓` / `j` | Navigate to the next task |
| `Space` | Toggle task completion status (`[✓]` / `[ ]`) |
| `a` | Enter task creation mode |
| `Enter` | Confirm and save new task (when in creation mode) |
| `Esc` | Cancel task creation |
| `d` / `Backspace` / `Delete` | Delete the currently selected task |
| `q` | Exit the application |

---

## Getting Started

### Prerequisites

- [Rust & Cargo](https://www.rust-lang.org/tools/install) (Edition 2024 or latest stable)

### Build and Run

```bash
# Clone the repository
git clone https://github.com/your-username/notemanager.git
cd notemanager

# Run in development mode
cargo run

# Build optimized release binary
cargo build --release
```

The compiled binary will be located in `target/release/notemanager`.

---

## Project Structure

```text
notemanager/
├── Cargo.toml          # Project configuration & dependencies
├── src/
│   ├── lib.rs          # Core domain models, persistence, and TaskManager logic
│   └── main.rs         # TUI event loop, keybinding dispatch, and Ratatui rendering
└── (Storage)           # Persisted automatically to the OS application data directory
```

---

## Disclaimer & Attribution

- **Core Library & Backend (`src/lib.rs`):** Data structures (`Task`, `TaskManager`, `Command`), input parser (`validate_user_input`), persistence routines, and task operations were designed and implemented by the author as a hands-on Rust learning project.
- **TUI Frontend (`src/main.rs`):** The terminal user interface, layout composition, and keyboard event loop using `ratatui` and `crossterm` were generated with AI assistance.
