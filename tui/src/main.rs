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

mod input {
    use std::io;
    use std::sync::mpsc;
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    };
    use std::thread;
    use std::time::Duration;

    use termion::event::Key;
    use termion::input::TermRead;

    #[derive(PartialEq, Eq)]
    pub enum Event<I> {
        Input(I),
        Tick,
        Exit,
    }

    /// A small event handler that wrap termion input and tick events. Each event
    /// type is handled in its own thread and returned to a common `Receiver`
    pub struct Events {
        rx: mpsc::Receiver<Event<Key>>,
        #[allow(dead_code)]
        input_handle: thread::JoinHandle<()>,
        #[allow(dead_code)]
        ignore_exit_key: Arc<AtomicBool>,
        #[allow(dead_code)]
        tick_handle: thread::JoinHandle<()>,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Config {
        pub exit_key: Key,
        pub tick_rate: Duration,
    }

    impl Default for Config {
        fn default() -> Config {
            Config {
                exit_key: Key::Char('q'),
                tick_rate: Duration::from_millis(50),
            }
        }
    }

    impl Events {
        pub fn new() -> Events {
            Events::with_config(Config::default())
        }

        pub fn with_config(config: Config) -> Events {
            let (tx, rx) = mpsc::channel();
            let ignore_exit_key = Arc::new(AtomicBool::new(false));
            let input_handle = {
                let tx = tx.clone();
                let ignore_exit_key = ignore_exit_key.clone();
                thread::spawn(move || {
                    let stdin = io::stdin();
                    for evt in stdin.keys() {
                        if let Ok(key) = evt {
                            let exit =
                                !ignore_exit_key.load(Ordering::Relaxed) && key == config.exit_key;
                            let ev = if exit { Event::Exit } else { Event::Input(key) };

                            if let Err(err) = tx.send(ev) {
                                eprintln!("{}", err);
                                return;
                            }

                            if exit {
                                return;
                            }
                        }
                    }
                })
            };

            let tick_handle = {
                thread::spawn(move || loop {
                    if tx.send(Event::Tick).is_err() {
                        break;
                    }
                    thread::sleep(config.tick_rate);
                })
            };
            Events {
                rx,
                ignore_exit_key,
                input_handle,
                tick_handle,
            }
        }

        pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
            self.rx.recv()
        }

        #[allow(dead_code)]
        pub fn disable_exit_key(&mut self) {
            self.ignore_exit_key.store(true, Ordering::Relaxed);
        }

        #[allow(dead_code)]
        pub fn enable_exit_key(&mut self) {
            self.ignore_exit_key.store(false, Ordering::Relaxed);
        }
    }
}

#[derive(FromArgs, Debug)]
#[argh(description = "top level")]
struct TopLevel {
    #[argh(positional)]
    project_path: String,
}

struct NavState {
    list_state: ListState,
    parent_file_id: Option<i64>,
}
impl NavState {
    fn new(parent_file_id: Option<i64>) -> NavState {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            list_state,
            parent_file_id,
        }
    }
}

struct InitializedState {
    index: assetindex::AssetIndex,
    nav_states: Vec<NavState>,
}

impl InitializedState {
    fn from_index(index: assetindex::AssetIndex) -> Self {
        Self {
            index,
            nav_states: vec![NavState::new(None)],
        }
    }
}

use tui::style::*;
use tui::text::{Span, Text};

impl InitializedState {
    fn cur_file(&self) -> Result<&AssetFile> {
        let idx = &self.index;
        let (_path, sample_guid) = idx.scene_guids()?.pop().unwrap();
        let file = idx.asset_by_guid(&sample_guid).unwrap();
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
                        s.nav_states.push(NavState::new(Some(selected)));
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
                .highlight_symbol(">");

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
        state = State::Initialized(InitializedState::from_index(index));
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
