//! This is a pretty direct port of the [sparkline demo in the ratatui crate][demo].
//!
//! [demo]: https://github.com/ratatui-org/ratatui/blob/main/examples/sparkline.rs

use console_error_panic_hook::set_once as set_panic_hook;
use crossterm::{execute, terminal::EnterAlternateScreen};
use rand::{
    distributions::{Distribution, Uniform},
    rngs::StdRng,
    SeedableRng,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Sparkline},
    Terminal as TuiTerminal,
};
use wasm_bindgen::prelude::*;
use web_sys::Crypto;
use xterm_js_sys::{
    crossterm_support::XtermJsCrosstermBackend,
    xterm::{LogLevel, Terminal, TerminalOptions, Theme},
};

use std::io::Write;

#[path = "../../common.rs"]
mod common;
use common::{log, AnimationFrameCallbackWrapper};

#[wasm_bindgen]
pub fn alt_run() -> Result<Option<AnimationFrameCallbackWrapper>, JsValue> {
    set_panic_hook();

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let terminal_div = document
        .get_element_by_id("terminal")
        .expect("should have a terminal div");

    let term = Terminal::new(Some(
        TerminalOptions::default()
            .with_log_level(LogLevel::Debug)
            .with_theme(Theme::nord())
            .with_font_family("'Fira Mono', monospace")
            .with_font_size(11.0),
    ));
    term.open(terminal_div);

    // Create animation frame wrappers without leaking
    let animation_wrapper = AnimationFrameCallbackWrapper::new();

    let mut counter = 0;
    let term_clone = term.clone();
    animation_wrapper.start(move || {
        log!("heyo! {}", counter);
        counter += 1;

        if counter % 10 == 0 {
            term_clone.write(&format!("ayo: {}\r\n", counter));
        }
        if counter % 600 == 0 {
            term_clone.reset();
        }

        counter < 3600 // Stop after 3600 iterations
    }).unwrap_or_else(|e| log!("Failed to start animation: {:?}", e));

    // Store the wrapper to prevent it from being dropped
    use std::sync::Mutex;
    static ANIMATION_WRAPPERS: Mutex<Vec<AnimationFrameCallbackWrapper>> = Mutex::new(Vec::new());

    if let Ok(mut wrappers) = ANIMATION_WRAPPERS.lock() {
        wrappers.push(animation_wrapper);
    }

    Ok(None)
}

#[derive(Clone)]
pub struct RandomSignal {
    distribution: Uniform<u64>,
    rng: StdRng,
}

impl RandomSignal {
    pub fn new(crypto: Crypto, lower: u64, upper: u64) -> RandomSignal {
        let mut seed = [0u8; 32];
        crypto.get_random_values_with_u8_array(&mut seed).unwrap();

        RandomSignal {
            distribution: Uniform::new(lower, upper),
            rng: StdRng::from_seed(seed),
        }
    }
}

impl Iterator for RandomSignal {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        Some(self.distribution.sample(&mut self.rng))
    }
}

struct App {
    signal: RandomSignal,
    data1: Vec<u64>,
    data2: Vec<u64>,
    data3: Vec<u64>,
}

impl App {
    fn new(crypto: Crypto) -> App {
        let mut signal = RandomSignal::new(crypto, 0, 100);
        let data1 = signal.by_ref().take(200).collect::<Vec<u64>>();
        let data2 = signal.by_ref().take(200).collect::<Vec<u64>>();
        let data3 = signal.by_ref().take(200).collect::<Vec<u64>>();
        App {
            signal,
            data1,
            data2,
            data3,
        }
    }

    fn update(&mut self) {
        let value = self.signal.next().unwrap();
        self.data1.pop();
        self.data1.insert(0, value);
        let value = self.signal.next().unwrap();
        self.data2.pop();
        self.data2.insert(0, value);
        let value = self.signal.next().unwrap();
        self.data3.pop();
        self.data3.insert(0, value);
    }
}

#[wasm_bindgen]
pub fn run() -> Result<Option<AnimationFrameCallbackWrapper>, JsValue> {
    set_panic_hook();

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let terminal_div = document
        .get_element_by_id("terminal")
        .expect("should have a terminal div");

    let term = Terminal::new(Some(
        TerminalOptions::default()
            .with_log_level(LogLevel::Debug)
            .with_theme(Theme::nord())
            .with_font_family("'Fira Mono', monospace")
            .with_font_size(11.0),
    ));
    term.open(terminal_div);

    let mut term_temp: XtermJsCrosstermBackend = (&term).into();
    execute!((&mut term_temp), EnterAlternateScreen).unwrap();
    drop(term_temp);

    // Store terminal in a static to avoid leaking but keep it alive
    use std::sync::Mutex;
    static TERMINAL_STORAGE: Mutex<Option<Terminal>> = Mutex::new(None);

    if let Ok(mut storage) = TERMINAL_STORAGE.lock() {
        *storage = Some(term);
        if let Some(ref stored_term) = *storage {
            let backend = CrosstermBackend::new(stored_term);
            let mut tui = TuiTerminal::new(backend).unwrap();
            tui.hide_cursor().unwrap();

            // Create default app state
            let mut app = App::new(window.crypto().unwrap());

            let main_loop = AnimationFrameCallbackWrapper::new();
            main_loop.start(move || {
                tui.draw(
                    |f| {
                        let chunks = Layout::default()
                            .direction(Direction::Vertical)
                            .margin(2)
                            .constraints(
                                [
                                    Constraint::Length(3),
                                    Constraint::Length(3),
                                    Constraint::Length(7),
                                    Constraint::Min(0),
                                ]
                                .as_ref(),
                            )
                            .split(f.size());
                        let sparkline = Sparkline::default()
                            .block(
                                Block::default()
                                    .title("Data1")
                                    .borders(Borders::LEFT | Borders::RIGHT),
                            )
                            .data(&app.data1)
                            .style(Style::default().fg(Color::Yellow));
                        f.render_widget(sparkline, chunks[0]);
                        let sparkline = Sparkline::default()
                            .block(
                                Block::default()
                                    .title("Data2")
                                    .borders(Borders::LEFT | Borders::RIGHT),
                            )
                            .data(&app.data2)
                            .style(Style::default().bg(Color::Green));
                        f.render_widget(sparkline, chunks[1]);
                        // Multiline
                        let sparkline = Sparkline::default()
                            .block(
                                Block::default()
                                    .title("Data3")
                                    .borders(Borders::LEFT | Borders::RIGHT),
                            )
                            .data(&app.data3)
                            .style(Style::default().fg(Color::Red));
                        f.render_widget(sparkline, chunks[2]);
                    },
                )
                .unwrap();

                app.update();
                true
            }).unwrap_or_else(|e| log!("Failed to start animation: {:?}", e));

            // Store the main loop to prevent it from being dropped
            static MAIN_LOOP_STORAGE: Mutex<Option<AnimationFrameCallbackWrapper>> = Mutex::new(None);
            if let Ok(mut storage) = MAIN_LOOP_STORAGE.lock() {
                *storage = Some(main_loop);
            }
        }
    }

    Ok(None)
}
