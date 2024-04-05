use crate::document;
use crate::row;
use crate::terminal;
use crate::Terminal;
use crate::Document;
use crate::Row;
use std::io::{stdin, stdout, Error, Write};
use std::cmp::min;
use std::env;
use std::time::{Instant, Duration};
use termion::{cursor::DetectCursorPos, event::Key, color};            
use termion::input::TermRead;
use termion::raw::IntoRawMode;

const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);
const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    document: Document,
    offset: Position,
    status_message: StatusMessage,
}

struct StatusMessage {
    time: Instant,
    text: String,
}

impl StatusMessage {
    fn from(message: String) -> Self {
        Self {
            time: Instant::now(),
            text: message,
        }
    }
}

impl Editor {
    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                error_handler(error);
            }

            if self.should_quit {
                break;
            }

            if let Err(error) = self.process_keypress() {
                error_handler(error);
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::cursor_hide();
        Terminal::clear_screen();
        Terminal::cursor_position(&Position::default());

        if self.should_quit {
            Terminal::clear_screen();
            println!("Kraj!");
        } else {
            self.draw_rows();
            self.draw_status_bar();
            self.draw_status_bar();
            Terminal::cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            });
        }

        Terminal::cursor_show();
        Terminal::flush()
    }

    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut initial_status = String::new();

        let document = if args.len() > 1 {
            let file_name = &args[1];

            let doc = Document::open(&file_name);
            if doc.is_ok() {
                doc.unwrap()
            } else {
                initial_status = format!("ERR: Could not open file: {}", file_name);
                Document::default()
            }
        } else {
            Document::default()
        };

        Self { 
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialie terminal"),
            document,
            cursor_position: Position::default(),
            offset: Position::default(),
            status_message: StatusMessage::from(initial_status),
        }
    }

    pub fn process_keypress(&mut self) -> Result<(), Error> {
        let key_pressed = Terminal::read_key()?;

        match key_pressed {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Char(c) => self.document.insert(&self.cursor_position, c),
            Key::Up | Key::Down | Key::Left | Key::Right => self.move_cursor(key_pressed),
            _ => (),
        }

        self.scroll();
        Ok(())
    }

    pub fn draw_row(&self, row: &Row) {
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x + width;
        let row: String = row.render(start, end);

        println!("{}\r", row)
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;

        for terminal_row in 0..height {
            Terminal::clear_current_line();

            if let Some(temp) = self.document.row(terminal_row as usize + self.offset.y) {
                self.draw_row(temp);
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome();
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_welcome(&self) {
        let mut msg = format!("Editor -- version {}", VERSION);
        let width = self.terminal.size().width as usize;
        let len = msg.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));

        msg = format!("~{}{}", spaces, msg);
        msg.truncate(width);

        println!("{}\r", msg);
    }

    fn draw_status_bar(&self) {
        let mut status;
        let width = self.terminal.size().width as usize;

        let mut file_name = "[No name]".to_string();

        if let Some(temp) = &self.document.file_name {
            file_name = temp.clone();
            file_name.truncate(20);
        }

        status = format!("{} - {} lines", file_name, self.document.len());

        let line_indicator = format!("{}/{}", self.cursor_position.y.saturating_add(1), self.document.len());
        let len = status.len() + line_indicator.len();

        if width > len {
            status.push_str(&"".repeat(width - len));
        }
        status.truncate(width);

        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_fg_color(STATUS_FG_COLOR);
        println!("{} \r", status);

        Terminal::reset_bg_color();
        Terminal::reset_fg_color();
    }

    fn draw_msg_bar(&self) {
        Terminal::clear_current_line();

        let msg = &self.status_message;

        if Instant::now() - msg.time < Duration::new(5, 0) {
            let mut text = msg.text.clone();
            text.truncate(self.terminal.size().width as usize);
            print!("{}", text);
        }
    }

    fn scroll(&mut self) {
        let Position {x, y} = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;

        let mut offset = &mut self.offset;

        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }

        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }

    fn move_cursor(&mut self, key: Key) {
        let terminal_height = self.terminal.size().height as usize;
        let Position{mut x, mut y} = self.cursor_position;
        let size = self.terminal.size();
        let height = self.document.len();
        let mut width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };

        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            },
            Key::Left => {
                if x > 0 {
                    x -= 1;
                } else if y > 0 {
                    y -= 1;

                    if let Some(row) = self.document.row(y) {
                        x = row.len();
                    } else {
                        x = 0;
                    }
                }
            },
            Key::Right => {
                if x < width {
                    x += 1;
                } else if y < height {
                    y += 1;
                    x = 0;
                }
            },
            // Key::Home => x = 0,
            // Key::End => x = width,
            // Key::PageUp => {
            //     if y > terminal_height {
            //         y = y - terminal_height;
            //     } else {
            //         y = 0;
            //     }
            // },
            // Key::PageDown => {
            //     if y.saturating_add(terminal_height) < height {
            //         y = y + terminal_height as usize;
            //     } else {
            //         y = height;
            //     }
            // },
            _ => (),
        }

        width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };

        if x > width {
            x = width;
        }

        self.cursor_position = Position{x, y}
    }

}

fn error_handler(e: Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}

    
fn read_key() -> Result<Key, Error> {
    loop {
        if let Some(key) = stdin().lock().keys().next() {
            return key;
        }
    }
}