use argh::FromArgs;
use std::{fmt::Display, io};
use termion::raw::IntoRawMode;
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout, Rect},
};

use tui::widgets::*;
use tui::Terminal;

use gen::*;

type Result<T> = anyhow::Result<T>;

mod input;

#[derive(FromArgs, Debug)]
#[argh(description = "top level")]
struct TopLevel {
    #[argh(positional)]
    project_path: String,
}

struct NavState {
    list_state: ListState,
    file_guid: String,
    parent_file_id: Option<i64>,
}

impl NavState {
    fn new(file_guid: String, parent_file_id: Option<i64>) -> NavState {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            list_state,
            file_guid,
            parent_file_id,
        }
    }
}

struct InitializedState {
    index: assetindex::AssetIndex,
    nav_states: Vec<NavState>,
}

impl InitializedState {
    fn new(index: assetindex::AssetIndex, file_guid: String) -> Self {
        Self {
            index,
            nav_states: vec![NavState::new(file_guid, None)],
        }
    }
}

use tui::style::*;
use tui::text::{Span, Text};

impl InitializedState {
    fn cur_file(&self) -> Result<&AssetFile> {
        let idx = &self.index;
        let state = self.nav_states.last().unwrap();

        let file = idx.asset_by_guid(&state.file_guid).unwrap();
        Ok(file)
    }

    fn parent_file_id(&self) -> Option<i64> {
        self.nav_states.last().and_then(|s| s.parent_file_id)
    }

    fn cur_nav_state_mut(&mut self) -> &mut NavState {
        self.nav_states.last_mut().unwrap()
    }

    fn cur_file_ids(&self) -> Result<Vec<i64>> {
        let file = self.cur_file()?;
        file.by_parent(self.parent_file_id())
    }

    fn child_count(&self, file_id: i64) -> Result<usize> {
        let file = self.cur_file()?;
        file.by_parent(Some(file_id)).map(|l| l.len())
    }

    fn handle_input(&mut self, key: termion::event::Key) {
        use termion::event::Key;

        let s = self;
        let file_ids = s.cur_file_ids().unwrap();
        let len = file_ids.len();

        let nav_state = s.cur_nav_state_mut();
        let cur_file_guid = nav_state.file_guid.clone();
        let list_state = &mut nav_state.list_state;

        let mut move_cursor = |forward: bool| {
            if let Some(idx) = list_state.selected() {
                let idx = if forward { idx + 1 } else { idx + len - 1 };
                list_state.select(Some(idx % len));
            } else if len > 0 {
                list_state.select(Some(0));
            }
        };

        match key {
            Key::Up => {
                move_cursor(false);
            }
            Key::Down => {
                move_cursor(true);
            }
            Key::Left | Key::Esc => {
                if s.nav_states.len() > 1 {
                    s.nav_states.pop();
                }
            }
            Key::Right | Key::Char('\n') => {
                if let Some(idx) = list_state.selected() {
                    let selected = file_ids[idx];

                    if s.child_count(selected).unwrap() > 0 {
                        s.nav_states
                            .push(NavState::new(cur_file_guid, Some(selected)));
                    }
                }
                //
            }
            _ => (),
        }
        //
    }

    fn render<B>(&mut self, f: &mut tui::Frame<B>, rect: Rect)
    where
        B: tui::backend::Backend,
    {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2), Constraint::Min(0)].as_ref())
            .split(rect);

        let idx = &self.index;

        let file = self.cur_file().unwrap();

        // header
        {
            let filename = file
                .guid()
                .and_then(|guid| idx.try_asset_path_by_guid(&guid))
                .and_then(|p| p.to_str())
                .unwrap_or("<unknown>");

            let text = format!("initialized path={} stats={}", filename, idx.dbg_stats());
            f.render_widget(
                Paragraph::new(Text::from(text)).wrap(Wrap { trim: false }),
                chunks[0],
            );
        }

        // body
        {
            let file_ids = self.cur_file_ids().unwrap();

            let mut list = Vec::new();
            for file_id in file_ids {
                let is_prefab = file
                    .object_by_file_id(file_id)
                    .unwrap()
                    .is_prefab_transform();

                let name = file
                    .name_by_file_id_ref(file_id, idx)
                    .unwrap_or("<unknown>".to_owned());
                let child_count = file.by_parent(Some(file_id)).map(|v| v.len()).unwrap_or(0);

                let annotation = Annotation {
                    is_prefab,
                    child_count,
                };

                list.push(ListItem::new(format!("{}{}", name, annotation)));
            }

            let items = List::new(list)
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol("> ");

            f.render_stateful_widget(items, chunks[1], &mut self.cur_nav_state_mut().list_state);
        }
    }
}

struct Annotation {
    is_prefab: bool,
    child_count: usize,
}

impl Display for Annotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_prefab {
            write!(f, " | p")
        } else if self.child_count > 0 {
            write!(f, " | c={}", self.child_count)
        } else {
            Ok(())
        }
    }
}

enum State {
    Uninitialized,
    #[allow(dead_code)]
    Initializing,
    #[allow(dead_code)]
    Initialized(InitializedState),
}

impl State {
    fn render<B>(&mut self, f: &mut tui::Frame<B>)
    where
        B: tui::backend::Backend,
    {
        let size = f.size();

        let block = Block::default().title("Block").borders(Borders::ALL);
        let inner = block.inner(size);
        f.render_widget(block, size);

        let body = match self {
            State::Uninitialized => Span::raw("uninitialized"),
            State::Initializing => Span::raw("initializing"),
            State::Initialized(ref mut s) => {
                s.render(f, inner);
                return;
            }
        };
        f.render_widget(Paragraph::new(body), inner);
        //
    }
}

fn main() -> Result<()> {
    use input::*;

    let args: TopLevel = argh::from_env();

    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let events = Events::new();
    let mut state = State::Uninitialized;

    if true {
        let index = assetindex::AssetIndex::from_path(&args.project_path)?;

        let (_path, sample_guid) = index.scene_guids()?.pop().unwrap();
        state = State::Initialized(InitializedState::new(index, sample_guid));
    }

    loop {
        terminal.draw(|f| state.render(f))?;

        let ev = events.next()?;
        match ev {
            Event::Input(key) => {
                if let State::Initialized(ref mut s) = state {
                    s.handle_input(key);
                }
            }
            Event::Exit => {
                break;
            }
            _ => (),
        }
    }

    terminal.clear()?;
    terminal.show_cursor()?;

    Ok(())
}
