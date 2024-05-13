// Helper Struct that defines the size of the terminal
pub struct Size {
    pub width: u16,
    pub height: u16,
}

// Struct for defining the Terminal configuration
pub struct Terminal {
    size: Size,
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
        })
    }

    // Declares the size to be returned as a read-only reference to our internal size
    pub fn size(&self) -> &Size {
        &self.size
    }
}
