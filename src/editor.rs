use crate::document;
use crate::row;
use crate::terminal;
use crate::Terminal;
use crate::Document;
use crate::Row;
use std::io::{stdin, stdout, Error, Write};
use std::cmp::min;   
use std::env;         
use termion::{cursor::DetectCursorPos, event::Key};            
use termion::input::TermRead;            
use termion::raw::IntoRawMode;

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
}

impl Editor {
    pub fn run(&mut self) {
        //let _std_output = stdout().into_raw_mode().unwrap();

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
        //print!("{}{}", termion::clear::All, termion::cursor::Goto(1,1));
        Terminal::cursor_hide();
        Terminal::clear_screen();
        Terminal::cursor_position(&Position::default());

        if self.should_quit {
            Terminal::clear_screen();
            println!("Kraj!");
        } else {
            self.draw_rows();
            //print!("{}", termion::cursor::Goto(1,1));
            Terminal::cursor_position(&self.cursor_position);
        }
        //stdout().flush()
        Terminal::cursor_show();
        Terminal::flush()
    }

    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let document = if args.len() > 1 {
            let file_name = &args[1];
            Document::open(&file_name).unwrap_or_default()
        } else {
            Document::default()
        };

        Self { 
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialie terminal"),
            document,
            cursor_position: Position::default(),
            offset: Position::default(),
        }
    }

    pub fn process_keypress(&mut self) -> Result<(), Error> {
        let key_pressed = Terminal::read_key()?;

        match key_pressed {
            Key::Ctrl('q') => self.should_quit = true,
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
        for terminal_row in 0..height - 1 {
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
        let Position{mut x, mut y} = self.cursor_position;
        let size = self.terminal.size();
        let (height, width) = (self.document.len(), size.width.saturating_sub(1) as usize);
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            },
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1);
                }
            },
            // Key::Home => x = 0,
            // Key::End => x = width,
            // Key::PageUp => y = 0,
            // Key::PageDown => y = height,
            _ => (),
        }

        self.cursor_position = Position{x, y}
    }

}

fn error_handler(e: Error) {
    //print!("{}", termion::clear::All);
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