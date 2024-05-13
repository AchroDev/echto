use std::io::{self, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

// Helper Struct that defines the size of the terminal
pub struct Size {
    pub width: u16,
    pub height: u16,
}

// Struct for defining the Terminal configuration
pub struct Terminal {
    size: Size,
    // Sets the terminal mode to raw instead of canonical
    _stdout: RawTerminal<std::io::Stdout>,
}

// Implementation of Terminal struct
impl Terminal {
    // Get terminal size and return the new instance of Terminal
    pub fn default() -> Result<Self, std::io::Error> {
        // Get termions terminal size
        let size = termion::terminal_size()?;
        // Wrapped in Ok to account for any potential errors
        Ok(Self {
            // Converting the terminal_size into a "Size"
            size: Size {
                width: size.0,
                height: size.1,
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }

    // Declares the size to be returned as a read-only reference to our internal size
    pub fn size(&self) -> &Size {
        &self.size
    }

    // Clears the screen when called
    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    // Resets the cursor position when called
    pub fn cursor_position(x: u16, y: u16) {
        let x = x.saturating_add(1);
        let y = y.saturating_add(1);
        print!("{}", termion::cursor::Goto(1, 1));
    }

    // When called, forces stdout to print out everything it has, either wrapping nothing or erroring
    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    // Reads the key being pressed
    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
}
