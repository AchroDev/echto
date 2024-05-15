#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;
use editor::Editor;
pub use editor::Position;
pub use terminal::Terminal;

// Entry point
fn main() {
    // Run Editor mod as default (defined in editor.rs)
    Editor::default().run();
}
