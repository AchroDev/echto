use crate::Document;
use crate::Row;
use crate::Terminal;
use std::env;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

// Struct for tracking the cursors x and y position
#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

// Struct for defining the text editors configuration
pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
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
        let args: Vec<String> = env::args().collect();
        let document = if args.len() > 1 {
            let file_name = &args[1];
            Document::open(&file_name).unwrap_or_default()
        } else {
            Document::default()
        };
        // The default state is NOT to quit
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initalize terminal"),
            document,
            cursor_position: Position::default(),
            offset: Position::default(),
        }
    }

    // Refreshes the screen on open and exit, also resetting the cursor position to the top left.
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position::default());
        if self.should_quit {
            Terminal::clear_screen();
            println!("Thanks for using Echto! - AchroDev\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            });
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    // Performs action based on read keypress
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up
            | Key::Down
            | Key::Left
            | Key::Right
            | Key::PageUp
            | Key::PageDown
            | Key::Home
            | Key::End => self.move_cursor(pressed_key),
            _ => (),
        }
        self.scroll();
        Ok(())
    }

    /* Checks for if the cursor moves outside of the visible window, and if so, adjusts the offset
     *  so that the cursor is just inside the visible window.
     */
    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let mut offset = &mut self.offset;
        if y < offset.y {
            offset.y = y;
        } else if y > offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }
        if x < offset.x {
            offset.x = x;
        } else if x > offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }

    // Moves the cursor along the x and y axis depending on which arrow key is used
    fn move_cursor(&mut self, key: Key) {
        let Position { mut y, mut x } = self.cursor_position;
        let size = self.terminal.size();
        let height = self.document.len();
        let width = size.width.saturating_add(1) as usize;
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1);
                }
            }
            Key::PageUp => y = 0,
            Key::PageDown => y = height,
            Key::Home => x = 0,
            Key::End => x = width,
            _ => (),
        }
        self.cursor_position = Position { x, y }
    }

    // Draws the welcome message
    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Echto text editor -- version {}\r", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }

    // Drawing row for current example text
    pub fn draw_row(&self, row: &Row) {
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x + width;
        let row = row.render(start, end);
        println!("{}\r", row);
    }

    // Handles drawing each row of the buffer of text being edited
    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height - 1 {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y) {
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message();
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
