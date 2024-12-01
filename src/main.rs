use std::io;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::time::Duration;

use clipboard::{ClipboardProvider, ClipboardContext};
use device_query::{DeviceEvents, DeviceState, Keycode};
use ratatui::prelude::*;
use ratatui::{DefaultTerminal, Frame};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Widget, Gauge, block::{self, Title}};
use ratatui::style::{Stylize, Color, Styled};
use ratatui::symbols::border;

use rdev::{self, simulate, SimulateError};

const TICK_RATE: u64 = 100;  // 1 tick : 50 ms
const KEY_INPUT_DELAY: u64 = 10;

fn main() -> io::Result<()> {
    let (tx_tick, rx_tick): (Sender<()>, Receiver<()>) = channel();
    let (tx_skill, rx_skill): (Sender<Keycode>, Receiver<_>) = channel();

    let device_state = DeviceState::new();
    let _guard = device_state.on_key_down(move |&key| {
        let mut clipboard = ClipboardContext::new().unwrap();

        match key {
            Keycode::Enter |
            Keycode::Key0 |
            Keycode::Key1 |
            Keycode::Key2 |
            Keycode::Key3 |
            Keycode::Key4 |
            Keycode::Key5 |
            Keycode::Key6 |
            Keycode::Key7 |
            Keycode::Key8 |
            Keycode::Key9 => {
                tx_skill.send(key).unwrap();
            }
            Keycode::Left |
            Keycode::Right |
            Keycode::Up |
            Keycode::Down => {}
            Keycode::F11 => {
                std::thread::spawn(move || {
                    clipboard.set_contents("abcd".to_owned()).unwrap();
                    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

                    simulate(&rdev::EventType::KeyPress(rdev::Key::MetaLeft));
                    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));
                    simulate(&rdev::EventType::KeyPress(rdev::Key::KeyV));
                    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));
                    simulate(&rdev::EventType::KeyRelease(rdev::Key::MetaLeft));
                    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));
                    simulate(&rdev::EventType::KeyRelease(rdev::Key::KeyV));
                });
            }
            Keycode::F12 => {
                std::thread::spawn(move || {
                    clipboard.set_contents("1234".to_owned()).unwrap();
                });
            }
            _ => {
                tx_skill.send(Keycode::Escape).unwrap();
            }
        }
    });

    let thread = std::thread::spawn(move || {
        let tick_rate = Duration::from_millis(TICK_RATE);

        loop {
            std::thread::sleep(tick_rate);
            tx_tick.send(()).unwrap();
        }
    });

    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal, &rx_tick, &rx_skill);

    ratatui::restore();

    thread.join().unwrap();

    app_result
}

struct App {
    tab: bool,
    init_time: Duration,
    cool_time: Duration,
    exit: bool,
}
impl Default for App {
    fn default() -> Self {
        App {
            tab: false,
            init_time: Duration::from_millis(10_500),
            cool_time: Duration::from_millis(0),
            exit: false,
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal, rx_tick: &Receiver<()>, rx_skill: &Receiver<Keycode>) -> io::Result<()> {
        while !self.exit {
            self.handle_event()?;

            if rx_tick.try_recv().is_ok() {
                self.cool_time = self.cool_time
                    .checked_sub(Duration::from_millis(TICK_RATE))
                    .unwrap_or(Duration::from_millis(0));
            }

            if let Ok(skill) = rx_skill.try_recv() {
                if skill == Keycode::Enter {
                    if self.tab && self.cool_time < Duration::from_millis(TICK_RATE)
                    {
                        self.cool_time = self.init_time;
                        self.tab = false;
                    }

                } else if skill == Keycode::Key1 {
                    self.tab = true;
                } else if skill == Keycode::Escape {
                    self.tab = false;
                }

            }

            terminal.draw(|frame| {
                self.draw(frame);
            })?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_event(&mut self) -> io::Result<()> {
        // if let event::Event::Key(key) = event::()? {
        //     if let event::KeyCode::Char('q') = key.code {
        //         self.exit = true;
        //     }
        // }

        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let header = Line::from(" Skill Cool Time ".bold());
        let footer = Line::from(vec![
            "주식회사 논현크루".gray().into(),
        ]);
        let block = Block::bordered()
            .title(header.centered())
            .title_bottom(footer.right_aligned())
            .border_set(border::ROUNDED);

        let ratio = self.cool_time.as_millis() as f64 / self.init_time.as_millis() as f64;
        let label = Span::styled(format!(
            "{:2}.{:1}s",
            self.cool_time.as_secs(),
            self.cool_time.subsec_millis() / 100,
        ), Style::default().fg(Color::White));


        let block = Block::bordered()
            .title(" Hellfire Cool Time ")
            .title(
                Title::from("논현크루")
                    .alignment(Alignment::Right)
                    .position(block::Position::Bottom),
            )
            .border_set(border::ROUNDED)
            .style(Style::default().fg(Color::LightGreen));

        if ratio == 0.0 {
            Gauge::default()
                .block(block)
                .light_green()
                .gauge_style(Style::default().bg(Color::LightGreen))
                .ratio(ratio)
                .label(label)
                .render(area, buf);
        } else {
            Gauge::default()
                .block(block)
                .light_green()
                .gauge_style(Style::default().fg(Color::LightGreen))
                .ratio(ratio)
                .label(label)
                .render(area, buf);
        }

    }
}