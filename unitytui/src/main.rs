use argh::FromArgs;
use std::{fmt::Display, io};
use termion::event::Key;
use termion::raw::IntoRawMode;
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout, Rect},
    text::Spans,
};

use tui::widgets::*;
use tui::Terminal;

use gen::*;

type Result<T> = anyhow::Result<T>;

mod helper;
use helper::*;
mod input;
mod select;
use select::*;

#[derive(FromArgs, Debug)]
#[argh(description = "top level")]
struct TopLevel {
    #[argh(positional)]
    project_path: String,
}

struct NavState {
    list_state: ListInputState,
    file_guid: String,
    parent_file_id: Option<i64>,
}

impl NavState {
    fn new(file_guid: String, parent_file_id: Option<i64>) -> NavState {
        Self {
            list_state: ListInputState::default(),
            file_guid,
            parent_file_id,
        }
    }
}

#[derive(Clone)]
struct PopupState {
    #[allow(unused)]
    file_guid: String,
    file_id: i64,
}

pub struct InitializedState {
    pub index: assetindex::AssetIndex,
    nav_states: Vec<NavState>,

    select_state: Option<StateReverseRef>,
    popup_state: Option<PopupState>,
}

impl InitializedState {
    fn new(index: assetindex::AssetIndex, file_guid: String) -> Self {
        let mut s = Self {
            index,
            nav_states: vec![NavState::new(file_guid, None)],

            select_state: None,
            popup_state: None,
        };

        s.hierarchy_set_len();
        s
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

    fn cur_guid(&self) -> Option<String> {
        self.nav_states.last().map(|s| s.file_guid.to_owned())
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

    fn show_detail(&mut self, selected_idx: usize) {
        let file_ids = self.cur_file_ids().unwrap();

        let nav_state = self.nav_states.last().unwrap();
        let popup_state = PopupState {
            file_guid: nav_state.file_guid.to_owned(),
            file_id: file_ids[selected_idx],
        };
        self.popup_state = Some(popup_state);
    }

    fn select_item(&mut self, selected_idx: usize) -> bool {
        let file_ids = self.cur_file_ids().unwrap();
        let selected = file_ids[selected_idx];

        let cur_file = self.cur_file().unwrap();

        let ret = if let Some(guid) = cur_file.prefab_source_guid(selected) {
            let guid = guid.to_owned();
            self.nav_states.push(NavState::new(guid, None));
            true
        // follow prefab?
        } else if self.child_count(selected).unwrap() > 0 {
            let guid = cur_file.guid().unwrap();
            let state = NavState::new(guid.to_owned(), Some(selected));
            self.nav_states.push(state);
            true
        } else {
            false
        };

        self.hierarchy_set_len();
        ret
    }

    fn handle_input(&mut self, key: termion::event::Key) {
        if let Some(mut s) = self.select_state.take() {
            match s.list.next_state(key) {
                Some(InputNextState::Selected(idx)) => {
                    let guid = s.items[idx].1.to_owned();
                    self.nav_states.push(NavState::new(guid, None));
                }
                Some(InputNextState::Escaped) => {}
                None => {
                    s.list.handle_input(key);
                    self.select_state = Some(s);
                }
            }
            return;
        }

        self.handle_input_hierarchy(key)
    }

    fn hierarchy_set_len(&mut self) {
        let list_len = self.cur_file_ids().unwrap().len();
        let nav_state = self.cur_nav_state_mut();
        let list_state = &mut nav_state.list_state;
        list_state.len = list_len;

        if list_state.l.selected().is_none() && list_len > 0 {
            list_state.l.select(Some(0));
        }
    }

    fn handle_input_hierarchy(&mut self, key: termion::event::Key) {
        let s = self;

        s.hierarchy_set_len();
        let list_state = &mut s.cur_nav_state_mut().list_state;

        match key {
            Key::Char('r') => {
                let cur_guid = s.cur_guid().unwrap();
                s.select_state = Some(s.state_reverse_ref(&cur_guid));
            }
            _ => match list_state.next_state(key) {
                Some(InputNextState::Selected(idx)) => {
                    if s.select_item(idx) {
                        s.popup_state = None;
                    }
                }
                Some(InputNextState::Escaped) => {
                    if s.popup_state.is_some() {
                        s.popup_state = None;
                    } else if s.nav_states.len() > 1 {
                        s.nav_states.pop();
                    }
                }
                None => {
                    list_state.handle_input(key);

                    if let Some(idx) = list_state.selected() {
                        s.show_detail(idx);
                    } else {
                        s.popup_state = None;
                    }
                }
            },
        }
    }

    fn render_detail<B>(&mut self, f: &mut tui::Frame<B>, rect: Rect, popup_state: &mut PopupState)
    where
        B: tui::backend::Backend,
    {
        let file = self.cur_file().unwrap();

        let obj = file.object_by_file_id(popup_state.file_id).unwrap();
        let yaml = obj.dbg_yaml().unwrap();

        let spans = yaml.split('\n').map(|s| Spans::from(s)).collect::<Vec<_>>();
        let p = Paragraph::new(spans);

        f.render_widget(p, rect);
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

            f.render_stateful_widget(items, chunks[1], &mut self.cur_nav_state_mut().list_state.l);
        }

        if let Some(mut popup_state) = self.popup_state.take() {
            helper::render_popup(f, rect, |f, r| {
                self.render_detail(f, r, &mut popup_state);
            });
            self.popup_state = Some(popup_state);
        }

        if let Some(mut select_state) = self.select_state.take() {
            helper::render_popup(f, rect, |f, r| {
                self.render_reverse_ref(f, r, &mut select_state);
            });
            self.select_state = Some(select_state);
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

        let body = match self {
            State::Uninitialized => Span::raw("uninitialized"),
            State::Initializing => Span::raw("initializing"),
            State::Initialized(ref mut s) => {
                s.render(f, size);
                return;
            }
        };
        f.render_widget(Paragraph::new(body), size);
        //
    }
}

fn main() -> Result<()> {
    use input::*;

    simplelog::CombinedLogger::init(vec![simplelog::WriteLogger::new(
        log::LevelFilter::Debug,
        simplelog::Config::default(),
        std::fs::File::create("out.log").unwrap(),
    )])
    .unwrap();

    let args: TopLevel = argh::from_env();

    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let events = Events::new();
    let mut state = State::Uninitialized;

    if true {
        let index = assetindex::AssetIndex::from_path(&args.project_path)?;

        let (_path, sample_guid) = index.scene_guids()?.first().unwrap().to_owned();
        state = State::Initialized(InitializedState::new(index, sample_guid));
    }

    loop {
        terminal.draw(|f| state.render(f))?;

        let ev = events.next()?;
        match ev {
            Event::Input(key) => match state {
                State::Initialized(ref mut s) => {
                    s.handle_input(key);
                }
                _ => {}
            },
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
