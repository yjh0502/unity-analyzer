use super::*;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::Frame;

pub fn render_popup<B: Backend, F>(f: &mut Frame<B>, rect: Rect, func: F)
where
    F: FnOnce(&mut Frame<B>, Rect),
{
    let block = Block::default().title("detail").borders(Borders::ALL);
    let area = helper::centered_rect(80, 80, rect);

    f.render_widget(Clear, area);
    func(f, block.inner(area));
    f.render_widget(block, area);
}

/// helper function to create a centered rect using up
/// certain percentage of the available rect `r`
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

#[derive(Default, Clone)]
pub struct ListInputState {
    pub l: ListState,
    pub len: usize,
    pub selected: bool,
}

pub enum InputNextState {
    Selected(usize),
    Escaped,
}

impl ListInputState {
    #[allow(unused)]
    pub fn from_selected(idx: usize) -> Self {
        let mut l = ListState::default();
        l.select(Some(idx));

        Self {
            l,
            len: idx,
            selected: true,
        }
    }

    pub fn next_state(&self, key: Key) -> Option<InputNextState> {
        if key == Key::Char('L') {
            if let Some(idx) = self.l.selected() {
                Some(InputNextState::Selected(idx))
            } else {
                None
            }
        } else if self.selected && key == Key::Char('l') {
            if let Some(idx) = self.l.selected() {
                Some(InputNextState::Selected(idx))
            } else {
                None
            }
        } else if !self.selected && key == Key::Char('h') {
            Some(InputNextState::Escaped)
        } else {
            None
        }
    }

    pub fn selected(&self) -> Option<usize> {
        if self.selected {
            self.l.selected()
        } else {
            None
        }
    }

    pub fn handle_input(&mut self, key: Key) {
        let len = self.len;
        let mut move_cursor = |forward: bool| {
            if let Some(idx) = self.l.selected() {
                let idx = if forward { idx + 1 } else { idx + len - 1 };
                self.l.select(Some(idx % len));
            } else if len > 0 {
                self.l.select(Some(0));
            }
        };

        match key {
            Key::Up | Key::Char('k') => {
                move_cursor(false);
            }
            Key::Down | Key::Char('j') => {
                move_cursor(true);
            }
            Key::Left | Key::Char('h') => {
                self.selected = false;
            }
            Key::Right | Key::Char('l') | Key::Char('\n') => {
                self.selected = true;
            }
            _ => (),
        }
    }
}
