use std::io::{self, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

// Struct for defining the text editors configuration
pub struct Editor {
    should_quit: bool,
}

// Implementation of Editor struct
impl Editor {
    // Allows this mod to be called ("ran") in main.rs with .run()
    pub fn run(&mut self) {
        // Sets the terminal mode to raw instead of canonical
        let _stdout = stdout().into_raw_mode().unwrap();

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
        Self { should_quit: false }
    }

    // Refreshes the screen on open and exit, also resetting the cursor position to the top left.
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        if self.should_quit {
            println!("Thanks for using Echto!\r");
        } else {
            self.draw_rows();
            print!("{}", termion::cursor::Goto(1, 1));
        }
        io::stdout().flush()
    }

    // Performs action based on read keypress
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }

    // Handles drawing each row of the buffer of text being edited
    fn draw_rows(&self) {
        for _ in 0..24 {
            println!("~\r");
        }
    }
}

// Reads the key being pressed
fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}

// Defines how the program should crash
fn die(e: &std::io::Error) {
    print!("{}", termion::clear::All);
    panic!("{}", e);
}
