use super::{View, ViewError};
use crossterm::cursor::MoveTo;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use std::io::{Stdout, Write};

const BOX_TOP_LEFT: char = '╔';
const BOX_TOP_RIGHT: char = '╗';
const BOX_BOTTOM_LEFT: char = '╚';
const BOX_BOTTOM_RIGHT: char = '╝';
const BOX_VERTICAL: char = '║';
const BOX_HORIZONTAL: char = '═';

pub struct ListView {
    elems: Vec<String>,
    current_selection: Option<usize>,
}

impl ListView {
    pub fn from(labels: &[&str]) -> Self {
        let mut v = vec![];
        for &l in labels {
            v.push(String::from(l));
        }

        ListView {
            elems: v,
            current_selection: None,
        }
    }
}

impl View for ListView {
    fn process_key(&mut self, ke: KeyEvent) -> Result<Option<Box<dyn View>>, ViewError> {
        match ke {
            KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::NONE,
            } => {
                self.current_selection = match self.current_selection {
                    None => Some(0),
                    Some(v) => Some((v + 1) % self.elems.len()),
                };
            }
            KeyEvent {
                code: KeyCode::Up,
                modifiers: KeyModifiers::NONE,
            } => {
                self.current_selection = match self.current_selection {
                    None => Some(0),
                    Some(v) => {
                        let n = self.elems.len();
                        Some((v + n - 1) % n)
                    }
                };
            }
            _ => {
                // do nothing
            }
        }

        Ok(None)
    }

    fn tick(&mut self) {}

    fn draw(&self, stdout: &mut Stdout, term_size: (u16, u16)) -> Result<(), ViewError> {
        let mut max_elem_width = 0;
        for e in &self.elems {
            let cs = e.chars().count();
            if cs > max_elem_width {
                max_elem_width = cs;
            }
        }

        let (term_cols, term_rows) = term_size;

        let box_height = self.elems.len() + 4;
        let box_width = max_elem_width + 4;

        let (mut s_x, mut s_y) = (0, 0);
        if box_height < term_rows as usize {
            s_y = (term_rows as usize) / 2 - box_height / 2;
        }
        if box_width < term_cols as usize {
            s_x = (term_cols as usize) / 2 - box_width / 2;
        }

        for i in 0..box_height {
            execute!(stdout, MoveTo(s_x as u16, s_y as u16 + i as u16))?;

            let mut box_row = String::from("");
            if i == 0 || i == box_height - 1 {
                box_row.push(if i == 0 {
                    BOX_TOP_LEFT
                } else {
                    BOX_BOTTOM_LEFT
                });
                for _ in 1..box_width - 1 {
                    box_row.push(BOX_HORIZONTAL);
                }
                box_row.push(if i == 0 {
                    BOX_TOP_RIGHT
                } else {
                    BOX_BOTTOM_RIGHT
                });

                write!(stdout, "{}", box_row)?;
            } else if i == 1 || i == box_height - 2 {
                box_row.push(BOX_VERTICAL);
                for _ in 1..box_width - 1 {
                    box_row.push(' ');
                }
                box_row.push(BOX_VERTICAL);

                write!(stdout, "{}", box_row)?;
            } else {
                write!(stdout, "{}", BOX_VERTICAL)?;
                execute!(stdout, MoveTo(s_x as u16 + 2, s_y as u16 + i as u16))?;

                let msg = &self.elems[i - 2];
                match self.current_selection {
                    Some(s) if s == i - 2 => {
                        execute!(
                            stdout,
                            SetForegroundColor(Color::Black),
                            SetBackgroundColor(Color::White),
                            Print(msg),
                            ResetColor
                        )?;
                    }
                    _ => write!(stdout, "{}", msg)?,
                }
                execute!(
                    stdout,
                    MoveTo(s_x as u16 + box_width as u16 - 1, s_y as u16 + i as u16)
                )?;
                write!(stdout, "{}", BOX_VERTICAL)?;
            }
        }

        Ok(())
    }
}
