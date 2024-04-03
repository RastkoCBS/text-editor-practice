use crate::Terminal;
use std::io::{stdin, stdout, Error, Write};            
use termion::{cursor::DetectCursorPos, event::Key};            
use termion::input::TermRead;            
use termion::raw::IntoRawMode;
pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
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
        Terminal::cursor_position(0, 0);

        if self.should_quit {
            Terminal::clear_screen();
            println!("Kraj!");
        } else {
            self.draw_rows();
            //print!("{}", termion::cursor::Goto(1,1));
            Terminal::cursor_position(0, 0);
        }
        //stdout().flush()
        Terminal::cursor_show();
        Terminal::flush()
    }

    pub fn default() -> Self {
        Self { 
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialie terminal")
        }
    }

    pub fn process_keypress(&mut self) -> Result<(), Error> {
        let key_pressed = Terminal::read_key()?;

        match key_pressed {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }

    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height - 1 {
            Terminal::clear_current_line();
            println!("~\r");
        }
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