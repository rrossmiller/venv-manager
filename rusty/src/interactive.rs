use std::io::{self, Write};

use crossterm::{
    cursor, event, execute, queue,
    style::{self, Stylize},
    terminal,
};

const ERR_STDOUT_WRITE: &str = "err writing to stdout";
pub struct Menu {
    pub prompt: String,
    pub cursor_pos: usize,
    pub menu_items: Vec<MenuItem>,
}
pub struct MenuItem {
    pub text: String,
    //todo callback?
}
enum UserChoice {
    Quit,
    Moved,
    Choice(usize),
}

impl Menu {
    /// Show the options, and get the user's choice and
    pub fn display(&mut self) -> i32 {
        execute!(
            io::stdout(),
            terminal::EnterAlternateScreen,
            cursor::MoveTo(0, 0),
            cursor::Hide
        )
        .unwrap();

        terminal::enable_raw_mode().unwrap();

        // initialize menu
        self.render_menu_items(false);
        // wait for the user to make a choice
        let mut user_choice = -1;
        loop {
            if let Some(input) = self.get_input() {
                match input {
                    UserChoice::Quit => break,
                    UserChoice::Choice(i) => {
                        // user_choice = self.menu_items[i].text.clone();
                        user_choice = i as i32;
                        break;
                    }
                    UserChoice::Moved => {
                        self.render_menu_items(true);
                    }
                }
            }
        }

        execute!(io::stdout(), terminal::LeaveAlternateScreen, cursor::Show).unwrap();
        terminal::disable_raw_mode().expect("issue undoing raw mode");
        user_choice
    }

    fn render_menu_items(&self, redraw: bool) {
        let mut out = io::stdout();

        // if this is not the first time rendering, move the cursor up
        if redraw {
            queue!(
                out,
                cursor::MoveToPreviousLine(1 + self.menu_items.len() as u16),
                terminal::Clear(terminal::ClearType::FromCursorDown)
            )
            .expect(ERR_STDOUT_WRITE);
        }

        // write the promt
        queue!(
            out,
            style::PrintStyledContent(self.prompt.clone().cyan()),
            cursor::MoveToNextLine(1)
        )
        .expect(ERR_STDOUT_WRITE);

        // write every menu item
        for (idx, e) in self.menu_items.iter().enumerate() {
            let elem;
            if idx == self.cursor_pos {
                // let t = format!("> {}", e.text);
                let t = format!("➤ {}", e.text);
                elem = style::PrintStyledContent(t.yellow());
            } else {
                let t = format!("  {}", e.text);
                elem = style::PrintStyledContent(t.white());
            }
            queue!(out, elem, cursor::MoveToNextLine(1)).expect(ERR_STDOUT_WRITE);
        }

        out.flush().expect(ERR_STDOUT_WRITE);
    }

    fn get_input(&mut self) -> Option<UserChoice> {
        if let Ok(event::Event::Key(e)) = event::read() {
            match e.code {
                event::KeyCode::Esc | event::KeyCode::Char('q') => return Some(UserChoice::Quit),
                event::KeyCode::Up | event::KeyCode::Char('k') => {
                    if self.cursor_pos > 0 {
                        self.cursor_pos -= 1;
                    }
                    return Some(UserChoice::Moved);
                }

                event::KeyCode::Down | event::KeyCode::Char('j') => {
                    if self.cursor_pos < self.menu_items.len() - 1 {
                        self.cursor_pos += 1;
                    }
                    return Some(UserChoice::Moved);
                }
                event::KeyCode::Enter => return Some(UserChoice::Choice(self.cursor_pos)),
                _ => {} // do nothing if it's not one of the previous keys
            }
        } else {
            return Some(UserChoice::Quit);
        }
        None
    }
}
