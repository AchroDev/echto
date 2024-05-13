use crate::Terminal;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

// Struct for defining the text editors configuration
pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

// Implementation of Editor struct
impl Editor {
    // Allows this mod to be called ("ran") in main.rs with .run()
    pub fn run(&mut self) {
        // Main loop during runtime
        loop {
            // Run fn refresh_screen and if there is an error, die with &error
            if let Err(error) = self.refresh_screen() {
                die(&error);
            }
            // If the user commands "quit", break the loop
            if self.should_quit {
                break;
            }
            // Run fn process_keypress and if there is an error, die with &error
            if let Err(error) = self.process_keypress() {
                die(&error);
            }
        }
    }

    // Defines the default state/configuration of self
    pub fn default() -> Self {
        // The default state is NOT to quit
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initalize terminal"),
        }
    }

    // Refreshes the screen on open and exit, also resetting the cursor position to the top left.
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(0, 0);
        if self.should_quit {
            Terminal::clear_screen();
            println!("Thanks for using Echto!\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(0, 0);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    // Performs action based on read keypress
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }

    // Handles drawing each row of the buffer of text being edited
    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for row in 0..height - 1 {
            Terminal::clear_current_line();
            if row == height / 3 {
                println!("Echto text editor -- version {}\r", VERSION)
            } else {
                println!("~\r");
            }
        }
    }
}

// Defines how the program should crash
fn die(e: &std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
