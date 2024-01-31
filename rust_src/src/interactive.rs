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
    f_menu_items: Vec<MenuItem>,
    pub searchable: bool,
}

#[derive(Clone)]
pub struct MenuItem {
    pub text: String,
    //todo callback so that this can be extended beyond just using the string in some other
    //function. That function can be passed in here and called when the user selects this item
}


enum UserChoice {
    Quit,
    Moved,
    Choice(usize),
    Search,
}

impl Menu {
    pub fn new(prompt: String, menu_items: Vec<MenuItem>, searchable: bool) -> Self {
        let s = Self {
            prompt,
            cursor_pos: 0,
            f_menu_items: menu_items.clone(),
            menu_items,
            searchable,
        };

        s
    }

    /// Show the options, and get the user's choice and
    pub fn display(&mut self) -> usize {
        execute!(
            io::stdout(),
            terminal::EnterAlternateScreen,
            cursor::MoveTo(0, 0),
            cursor::Hide
        )
        .unwrap();

        terminal::enable_raw_mode().unwrap();

        // initialize menu
        let mut out = io::stdout();
        self.render_menu_items(&mut out, false, false);
        // wait for the user to make a choice
        let mut user_choice = 0;
        loop {
            if let Some(input) = self.get_input() {
                match input {
                    UserChoice::Quit => break,
                    UserChoice::Choice(i) => {
                        // user_choice = self.menu_items[i].text.clone();
                        user_choice = i;
                        break;
                    }
                    UserChoice::Moved => {
                        self.render_menu_items(&mut out, true, false);
                    }
                    UserChoice::Search => {
                        if self.searchable {
                            // shift everything down one
                            self.render_menu_items(&mut out, true, true);
                            queue!(
                                out,
                                cursor::MoveTo(0, 1),
                                style::PrintStyledContent(format!(">").white()),
                            )
                            .expect(ERR_STDOUT_WRITE);
                            out.flush().expect(ERR_STDOUT_WRITE);

                            // show the search input until the user presses enter
                            let mut searching = true;
                            let mut query = String::new();
                            while searching {
                                searching = self.get_search_input(&mut query);
                                // shift everything down one to show the search bar
                                self.filter_menu_items(&query);
                                self.render_menu_items(&mut out, true, true);
                                queue!(
                                    out,
                                    cursor::MoveTo(0, 1),
                                    style::PrintStyledContent(format!("> {}", query).white()),
                                )
                                .expect(ERR_STDOUT_WRITE);
                                out.flush().expect(ERR_STDOUT_WRITE);
                            }

                            // delete the search line
                            self.render_menu_items(&mut out, true, false);
                        }
                    }
                }
            }
        }

        execute!(io::stdout(), terminal::LeaveAlternateScreen, cursor::Show).unwrap();
        terminal::disable_raw_mode().expect("issue undoing raw mode");
        user_choice
    }

    fn filter_menu_items(&mut self, query: &str) {
        self.menu_items = self.f_menu_items.clone();
        self.menu_items.retain(|e| e.text.starts_with(query));
    }

    fn render_menu_items(&self, out: &mut io::Stdout, redraw: bool, active_search: bool) {
        // if this is not the first time rendering, move the cursor up
        if redraw {
            queue!(
                out,
                // cursor::MoveToPreviousLine(1 + self.menu_items.len() as u16),
                cursor::MoveTo(0, 0),
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

        // if search is active make room for the search bar
        if active_search {
            queue!(out, cursor::MoveToNextLine(1)).expect(ERR_STDOUT_WRITE);
        }

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
                // exit
                event::KeyCode::Esc => return Some(UserChoice::Quit),
                // move up
                event::KeyCode::Up | event::KeyCode::Char('k') => {
                    if self.cursor_pos > 0 {
                        self.cursor_pos -= 1;
                    } else {
                        self.cursor_pos = self.menu_items.len() - 1;
                    }
                    return Some(UserChoice::Moved);
                }
                // move down
                event::KeyCode::Down | event::KeyCode::Char('j') => {
                    if self.cursor_pos < self.menu_items.len() - 1 {
                        self.cursor_pos += 1;
                    } else {
                        self.cursor_pos = 0;
                    }
                    return Some(UserChoice::Moved);
                }
                // search
                event::KeyCode::Char('s') => {
                    return Some(UserChoice::Search);
                }
                // submit
                event::KeyCode::Enter => return Some(UserChoice::Choice(self.cursor_pos)),
                _ => {} // do nothing if it's not one of the previous keys
            }
        } else {
            return Some(UserChoice::Quit);
        }
        None
    }

    fn get_search_input(&self, query: &mut String) -> bool {
        if let Ok(event::Event::Key(e)) = event::read() {
            match e.code {
                // submit the search
                event::KeyCode::Enter => {
                    return false;
                }
                // exit without applying search
                event::KeyCode::Esc => {
                    query.clear();
                    return false;
                }
                // add the key to the query if it's alphanumeric
                event::KeyCode::Char(c) => {
                    query.push(c);
                    return true;
                }
                // delete the last char if backspace
                event::KeyCode::Backspace => {
                    query.pop();
                    return true;
                }
                // do nothing if it's not one of the previous keys
                _ => {
                    return true;
                }
            }
        }
        return true;
    }
}
