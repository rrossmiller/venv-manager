use std::io::{self, Write};

use crossterm::{
    cursor, event, execute, queue,
    style::{self, Stylize},
    terminal,
};

const ERR_STDOUT_WRITE: &str = "err writing to stdout";
struct Menu {
    prompt: String,
    cursor_pos: usize,
    menu_items: Vec<MenuItem>,
}
struct MenuItem {
    text: String,
    //todo sub_menu
}
enum UserChoice {
    Quit,
    Moved,
    Choice(usize),
}
impl Menu {
    pub fn display(&mut self) -> &str {
        terminal::enable_raw_mode().unwrap();

        // initialize menu
        self.render_menu_items(false);
        // wait for the user to make a choice
        loop {
            if let Some(input) = self.get_input() {
                match input {
                    UserChoice::Quit => break,
                    UserChoice::Choice(i) => {
                        println!("choice idx: {}\nchoice: {}", i, self.menu_items[i].text);
                        println!("TODO: do something with the choice");
                        break;
                    }
                    UserChoice::Moved => {}
                }
                self.render_menu_items(true);
            }
        }

        terminal::disable_raw_mode().expect("issue undoing raw mode");
        ""
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
                _ => {}
            }
        } else {
            return Some(UserChoice::Quit);
        }
        None
    }
}

pub fn start() {
    let menu = vec![
        MenuItem {
            text: "Activate".to_string(),
        },
        MenuItem {
            text: "Create".to_string(),
        },
        MenuItem {
            text: "Delete".to_string(),
        },
    ];

    execute!(
        io::stdout(),
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0),
        cursor::Hide
    )
    .unwrap();

    let mut menu = Menu {
        prompt: "This is a prompt:".to_string(),
        cursor_pos: 0,
        menu_items: menu,
    };
    menu.display();
    execute!(io::stdout(), cursor::Show).unwrap();
}
