use std::io;
use std::io::BufRead;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::time::{Duration, Instant};

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

const KEY_INPUT_DELAY: u64 = 20;


fn key_press_say_macro() {
    simulate(&rdev::EventType::KeyPress(rdev::Key::Alt));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    simulate(&rdev::EventType::KeyPress(rdev::Key::Kp1));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    simulate(&rdev::EventType::KeyRelease(rdev::Key::Kp1));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    std::thread::sleep(Duration::from_millis(20));

    simulate(&rdev::EventType::KeyPress(rdev::Key::Kp3));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    simulate(&rdev::EventType::KeyRelease(rdev::Key::Kp3));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    simulate(&rdev::EventType::KeyRelease(rdev::Key::Alt));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));
}

fn key_press_skill_O() {
    simulate(&rdev::EventType::KeyPress(rdev::Key::ShiftLeft));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    simulate(&rdev::EventType::KeyPress(rdev::Key::KeyZ));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    simulate(&rdev::EventType::KeyRelease(rdev::Key::KeyZ));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    simulate(&rdev::EventType::KeyRelease(rdev::Key::ShiftLeft));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    std::thread::sleep(Duration::from_millis(50));

    simulate(&rdev::EventType::KeyPress(rdev::Key::ShiftLeft));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    simulate(&rdev::EventType::KeyPress(rdev::Key::KeyO));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    simulate(&rdev::EventType::KeyRelease(rdev::Key::KeyO));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    simulate(&rdev::EventType::KeyRelease(rdev::Key::ShiftLeft));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    simulate(&rdev::EventType::KeyPress(rdev::Key::KeyD));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    simulate(&rdev::EventType::KeyRelease(rdev::Key::KeyD));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    simulate(&rdev::EventType::KeyPress(rdev::Key::KeyN));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    simulate(&rdev::EventType::KeyRelease(rdev::Key::KeyN));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    std::thread::sleep(Duration::from_millis(100));

    simulate(&rdev::EventType::KeyPress(rdev::Key::Return));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    simulate(&rdev::EventType::KeyRelease(rdev::Key::Return));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));
}


fn key_press_skill_P() {
    simulate(&rdev::EventType::KeyPress(rdev::Key::ShiftLeft));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    simulate(&rdev::EventType::KeyPress(rdev::Key::KeyZ));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    simulate(&rdev::EventType::KeyRelease(rdev::Key::KeyZ));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    simulate(&rdev::EventType::KeyRelease(rdev::Key::ShiftLeft));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    std::thread::sleep(Duration::from_millis(50));

    simulate(&rdev::EventType::KeyPress(rdev::Key::ShiftLeft));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    simulate(&rdev::EventType::KeyPress(rdev::Key::KeyP));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    simulate(&rdev::EventType::KeyRelease(rdev::Key::KeyP));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    simulate(&rdev::EventType::KeyRelease(rdev::Key::ShiftLeft));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    simulate(&rdev::EventType::KeyPress(rdev::Key::Kp4));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    simulate(&rdev::EventType::KeyRelease(rdev::Key::Kp4));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    std::thread::sleep(Duration::from_millis(100));

    simulate(&rdev::EventType::KeyPress(rdev::Key::Return));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    simulate(&rdev::EventType::KeyRelease(rdev::Key::Return));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));
}

fn key_press_skill_m() {
    simulate(&rdev::EventType::KeyPress(rdev::Key::ShiftLeft));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    simulate(&rdev::EventType::KeyPress(rdev::Key::KeyZ));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    simulate(&rdev::EventType::KeyRelease(rdev::Key::KeyZ));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    simulate(&rdev::EventType::KeyRelease(rdev::Key::ShiftLeft));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    std::thread::sleep(Duration::from_millis(50));

    simulate(&rdev::EventType::KeyPress(rdev::Key::KeyM));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    simulate(&rdev::EventType::KeyRelease(rdev::Key::KeyM));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));
}

fn key_press_item_n() {
    simulate(&rdev::EventType::KeyPress(rdev::Key::ControlLeft));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    simulate(&rdev::EventType::KeyPress(rdev::Key::KeyN));
    std::thread::sleep(Duration::from_millis(5));

    simulate(&rdev::EventType::KeyRelease(rdev::Key::KeyN));
    std::thread::sleep(Duration::from_millis(5));

    simulate(&rdev::EventType::KeyRelease(rdev::Key::ControlLeft));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));
}

fn key_press_paste() {
    simulate(&rdev::EventType::KeyPress(rdev::Key::ControlLeft));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    simulate(&rdev::EventType::KeyPress(rdev::Key::KeyV));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    simulate(&rdev::EventType::KeyRelease(rdev::Key::KeyV));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    simulate(&rdev::EventType::KeyRelease(rdev::Key::ControlLeft));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    std::thread::sleep(Duration::from_millis(50));

    simulate(&rdev::EventType::KeyPress(rdev::Key::Return));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));

    simulate(&rdev::EventType::KeyRelease(rdev::Key::Return));
    std::thread::sleep(Duration::from_millis(KEY_INPUT_DELAY));
}



fn main() -> io::Result<()> {
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
            Keycode::F7 => {
                std::thread::spawn(move || {
                    key_press_skill_O();
                    std::thread::sleep(Duration::from_millis(500));
                    key_press_say_macro();
                    std::thread::sleep(Duration::from_millis(500));
                    key_press_say_macro();
                });
            }
            Keycode::F8 => {
                std::thread::spawn(move || {
                    if let Ok(file) = std::fs::File::open("id.txt") {
                        let reader = std::io::BufReader::new(file);

                        if let Some(first_line) = reader.lines().nth(0) {
                            if let Ok(first) = first_line {
                                if clipboard.set_contents(first.to_owned()).is_ok() {
                                    key_press_item_n();
                                    std::thread::sleep(Duration::from_millis(500));
                                    key_press_skill_m();
                                    std::thread::sleep(Duration::from_millis(100));
                                    key_press_paste();
                                }
                            }
                        }
                    }
                });
            }
            Keycode::F9 => {
                std::thread::spawn(move || {
                    if let Ok(file) = std::fs::File::open("id.txt") {
                        let reader = std::io::BufReader::new(file);

                        if let Some(first_line) = reader.lines().nth(1) {
                            if let Ok(first) = first_line {
                                if clipboard.set_contents(first.to_owned()).is_ok() {
                                    key_press_skill_P();
                                    //key_press_item_n();
                                    std::thread::sleep(Duration::from_millis(500));
                                    key_press_skill_m();
                                    std::thread::sleep(Duration::from_millis(100));
                                    key_press_paste();
                                }
                            }
                        }
                    }
                });
            }
            _ => {
                tx_skill.send(Keycode::Escape).unwrap();
            }
        }
    });

    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal, &rx_skill);

    ratatui::restore();

    app_result
}

struct App {
    tab: bool,
    cool_time: Duration,
    fired_at: Instant,
    exit: bool,
}
impl Default for App {
    fn default() -> Self {
        App {
            tab: false,
            cool_time: Duration::from_millis(10_250),
            fired_at: Instant::now() - Duration::from_millis(10_250),
            exit: false,
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal, rx_skill: &Receiver<Keycode>) -> io::Result<()> {
        while !self.exit {
            // self.handle_event()?;

            if let Ok(skill) = rx_skill.try_recv() {
                match skill {
                    Keycode::Enter => {
                        let duration = Instant::now().saturating_duration_since(self.fired_at);
                        if self.tab && duration >= self.cool_time
                        {
                            self.fired_at = Instant::now();
                            self.tab = false;
                        }
                    }
                    Keycode::Key1 => {
                        self.tab = true;
                    }
                    Keycode::Escape => {
                        self.tab = false;
                    }
                    _ => {}
                };
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

        let duration = Instant::now().saturating_duration_since(self.fired_at);
        let rest = self.cool_time.checked_sub(duration).unwrap_or(Duration::from_millis(0));

        let ratio = rest.as_millis() as f64 / self.cool_time.as_millis() as f64;
        let label = Span::styled(format!(
            "{:2}.{:1}s",
            rest.as_secs(),
            rest.subsec_millis() / 100,
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