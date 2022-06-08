use tui::style::{Modifier, Style};
use tui::text::{Span, Spans, Text};
use tui::widgets::{Block, Paragraph, Widget};

pub enum Key {
    Char(char),
    Backspace,
    Enter,
    Left,
    Right,
    Up,
    Down,
    Tab,
    Delete,
    Home,
    End,
    Null,
}

pub struct Input {
    pub key: Key,
    pub ctrl: bool,
}

impl Default for Input {
    fn default() -> Self {
        Input {
            key: Key::Null,
            ctrl: false,
        }
    }
}

impl From<crossterm::event::Event> for Input {
    fn from(event: crossterm::event::Event) -> Self {
        if let crossterm::event::Event::Key(key) = event {
            Self::from(key)
        } else {
            Self::default()
        }
    }
}

impl From<crossterm::event::KeyEvent> for Input {
    fn from(key: crossterm::event::KeyEvent) -> Self {
        use crossterm::event::{KeyCode, KeyModifiers};
        let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
        let key = match key.code {
            KeyCode::Char(c) => Key::Char(c),
            KeyCode::Backspace => Key::Backspace,
            KeyCode::Enter => Key::Enter,
            KeyCode::Left => Key::Left,
            KeyCode::Right => Key::Right,
            KeyCode::Up => Key::Up,
            KeyCode::Down => Key::Down,
            KeyCode::Tab => Key::Tab,
            KeyCode::Delete => Key::Delete,
            KeyCode::Home => Key::Home,
            KeyCode::End => Key::End,
            _ => Key::Null,
        };
        Self { key, ctrl }
    }
}

pub struct TextArea<'a> {
    lines: Vec<String>,
    block: Option<Block<'a>>,
    style: Style,
    cursor: (usize, usize),
}

impl<'a> Default for TextArea<'a> {
    fn default() -> Self {
        Self {
            lines: vec![" ".to_string()],
            block: None,
            style: Style::default(),
            cursor: (0, 0),
        }
    }
}

impl<'a> TextArea<'a> {
    pub fn input(&mut self, input: impl Into<Input>) {
        let input = input.into();
        if input.ctrl {
            // TODO
        } else {
            match input.key {
                Key::Char(c) => self.insert_char(c),
                Key::Backspace => self.delete_char(),
                _ => {}
            }
        }
    }

    pub fn insert_char(&mut self, c: char) {
        let (row, col) = self.cursor;
        let line = &mut self.lines[row];
        if let Some((i, _)) = line.char_indices().nth(col) {
            line.insert(i, c);
            self.cursor.1 += 1;
        }
    }

    pub fn delete_char(&mut self) {
        let (row, col) = self.cursor;
        let line = &mut self.lines[row];
        if col == 0 {
            return;
        }
        if let Some((i, _)) = line.char_indices().nth(col - 1) {
            line.remove(i);
            self.cursor.1 -= 1;
        }
    }

    pub fn widget(&'a self) -> impl Widget + 'a {
        let mut lines = Vec::with_capacity(self.lines.len());
        for (i, l) in self.lines.iter().enumerate() {
            if i == self.cursor.0 {
                let (i, c) = l
                    .char_indices()
                    .nth(self.cursor.1)
                    .unwrap_or((l.len() - 1, ' '));
                let j = i + c.len_utf8();
                lines.push(Spans::from(vec![
                    Span::from(&l[..i]),
                    Span::styled(&l[i..j], Style::default().add_modifier(Modifier::REVERSED)),
                    Span::from(&l[j..]),
                ]));
            } else {
                lines.push(Spans::from(l.as_str()));
            }
        }
        let mut p = Paragraph::new(Text::from(lines)).style(self.style);
        if let Some(b) = self.block.clone() {
            p = p.block(b);
        }
        p
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
}
