#![warn(clippy::all, clippy::pedantic)]
mod editor;
use editor::Editor;

// Entry point
fn main() {
    // Run Editor mod as default (defined in editor.rs)
    Editor::default().run();
}
