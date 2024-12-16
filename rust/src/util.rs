use crossterm::{
    cursor::{MoveDown, MoveTo, MoveToColumn},
    event::{poll, read, Event, KeyCode, KeyModifiers, ModifierKeyCode},
    execute, queue,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::io::{self, Write};
use std::time::Duration;
use std::{collections::VecDeque, process};

pub struct Framebuffer {
    buffer: VecDeque<String>,
    capacity: usize,
    debug_on: bool,
    current_frame_index: Option<usize>,
}

impl Framebuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::new(),
            capacity,
            debug_on: false,
            current_frame_index: None,
        }
    }

    pub fn push(&mut self, frame: String) {
        if self.buffer.len() == self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(frame);
        self.current_frame_index = Some(self.buffer.len() - 1); // this is safe because len is at least 1 now
    }

    pub fn debug(&mut self) -> std::io::Result<()> {
        if self.buffer.is_empty() {
            return Ok(());
        }
        let mut stdout = io::stdout();

        enable_raw_mode()?;
        queue!(stdout, Clear(ClearType::All), MoveTo(0, 0));

        if self.debug_on {
            self.debug_print();
        } else {
            if Self::space_has_been_pressed()? {
                self.debug_on = true;
                self.debug_print();
            } else {
                Self::print_frame(self.current_frame(), &self.debug_off_footer());
            }
        };

        disable_raw_mode()?;
        Ok(())
    }

    fn debug_print(&mut self) -> std::io::Result<()> {
        let mut stdout = io::stdout();
        Self::print_frame(&self.current_frame(), &self.debug_on_footer());
        loop {
            if let Event::Key(key_event) = read()? {
                match key_event.code {
                    KeyCode::Esc => {
                        self.debug_on = false;
                        break;
                    }
                    KeyCode::Up => {
                        let current_frame_index = self.current_frame_index.expect("debug_print is internal, and shouldn't have been called with no available frames.");
                        let next_frame_index = current_frame_index + 1;
                        let Some(next_frame) = self.buffer.get(next_frame_index) else {
                            // return control to the caller, but leave debug mode on -- this means next time the caller
                            // calls `debug`, we'll print the newest frame and pause again.
                            break;
                        };
                        self.current_frame_index = Some(next_frame_index);
                        Self::print_frame(next_frame, &self.debug_on_footer());
                        stdout.flush()?;
                    }
                    KeyCode::Down => {
                        let current_frame_index = self.current_frame_index.expect("debug_print is internal, and shouldn't have been called with no available frames.");
                        let prior_frame_index = if current_frame_index > 0 {
                            current_frame_index - 1
                        } else {
                            current_frame_index
                        };
                        let Some(prior_frame) = self.buffer.get(prior_frame_index) else {
                            panic!("This should've been impossible");
                        };
                        self.current_frame_index = Some(prior_frame_index);
                        Self::print_frame(prior_frame, &self.debug_on_footer());
                        stdout.flush()?;
                    }
                    KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                        disable_raw_mode()?;
                        process::exit(130); // 130 is the typical exit code for SIGINT
                    }
                    _ => {}
                };
            };
        }
        Ok(())
    }

    /// Will panic if there's no current frame.
    fn current_frame(&self) -> &String {
        self.buffer.get(self.current_frame_index.unwrap()).unwrap()
    }

    fn footer(&self, text: &str) -> String {
        format!(
            "{} (frame {}/{})",
            text,
            self.current_frame_index.unwrap() + 1,
            self.buffer.len()
        )
    }

    fn debug_off_footer(&self) -> String {
        self.footer("Press Space to enter debug mode")
    }

    fn debug_on_footer(&self) -> String {
        self.footer("Press Esc to exit debug mode, or the Up and Down arrows to navigate frames")
    }

    fn space_has_been_pressed() -> std::io::Result<bool> {
        while poll(Duration::from_millis(0))? {
            if let Event::Key(key_event) = read()? {
                if key_event.code == KeyCode::Char(' ') {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    fn print_frame(frame: &str, footer: &str) {
        let mut stdout = io::stdout();
        queue!(stdout, Clear(ClearType::All), MoveTo(0, 0));
        for (y, line) in frame.lines().enumerate() {
            queue!(stdout, Print(line), MoveTo(0, y as u16),);
        }
        queue!(stdout, MoveDown(2), MoveToColumn(0), Print(footer));
        stdout.flush();
    }
}
