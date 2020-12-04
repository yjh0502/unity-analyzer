use argh::FromArgs;
use std::io;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;

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
                tick_rate: Duration::from_millis(250),
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

struct InitializedState {
    index: assetindex::AssetIndex,
    list_state: ListState,

    parent_file_id: Option<i64>,
}

impl InitializedState {
    fn from_index(index: assetindex::AssetIndex) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            index,
            list_state,
            parent_file_id: None,
        }
    }
}

use tui::layout::*;
use tui::style::*;
use tui::text::{Span, Text};

impl InitializedState {
    fn render<B>(&mut self, f: &mut tui::Frame<B>, rect: Rect)
    where
        B: tui::backend::Backend,
    {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0)].as_ref())
            .split(rect);

        let idx = &self.index;

        let sample_guid = idx.scene_guids().unwrap().pop().unwrap();
        let file = idx.asset_by_guid(&sample_guid).unwrap();
        let file_ids = file.by_parent(self.parent_file_id).unwrap();

        // header
        {
            let text = format!("initialized stats={}", idx.dbg_stats());
            f.render_widget(
                Paragraph::new(Text::from(text)).wrap(Wrap { trim: false }),
                chunks[0],
            );
        }

        // body
        let mut list = Vec::new();
        for file_id in file_ids {
            let name = file.name_by_file_id(file_id).unwrap_or("<unknown>");

            list.push(ListItem::new(name.to_owned()));
        }

        let items = List::new(list)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">");

        f.render_stateful_widget(items, chunks[1], &mut self.list_state);
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

        use termion::event::Key;
        match events.next()? {
            Event::Input(Key::Char(_ch)) => {
                //
                ()
            }
            Event::Input(Key::Down) => {
                if let State::Initialized(ref mut s) = state {
                    let len = s.index.scene_guids()?.len();
                    if let Some(idx) = s.list_state.selected() {
                        s.list_state.select(Some((idx + 1) % len));
                    } else if len > 0 {
                        s.list_state.select(Some(0));
                    }
                }
            }
            Event::Exit => {
                break;
            }
            _ => (),
        }
    }

    if false {}

    terminal.clear()?;
    terminal.show_cursor()?;

    Ok(())
}
