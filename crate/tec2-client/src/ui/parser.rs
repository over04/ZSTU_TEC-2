use crate::app::{AppUtil, Page};
use lalrpop_util::ParseError;
use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::Alignment;
use ratatui::prelude::{Constraint, Layout, Position, Text};
use ratatui::widgets::{Block, List, ListItem, Paragraph, Wrap};
use std::cmp::{max, min};
use std::fmt::{Display, Formatter};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tec2_parser::grammar;
use tec2_parser::parser::parser::ExprParser;

const USER_INPUT_PREFIX: &str = ">>> ";

pub struct Parser {
    util: AppUtil,
    cursor: (usize, usize),
    max_cursor: usize,
    user_input: Arc<Mutex<String>>,
    controller_history: Arc<Mutex<Vec<String>>>,
    last_parse_result: Arc<Mutex<Option<ParseResult>>>,
    run_flag: Arc<AtomicBool>,
}

#[derive(Debug)]
enum ParseResult {
    InvalidToken {
        location: usize,
    },
    UnrecognizedEof {
        location: usize,
        expected: Vec<String>,
    },
    UnrecognizedToken {
        token: (usize, String, usize),
        expected: Vec<String>,
    },
    ExprParseError(tec2_parser::error::Error),
    Result(String),
}

impl Display for ParseResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseResult::InvalidToken { location } => {
                write!(f, "在 \" {} \" 上出现不合法的token", location)
            }
            ParseResult::UnrecognizedEof { location, expected } => {
                write!(
                    f,
                    "期望在 \" {} \" 获得Token: [{}]",
                    location,
                    expected.join(", ")
                )
            }
            ParseResult::UnrecognizedToken { token, expected } => {
                write!(
                    f,
                    "期望获得Token: [{}], 但是获取到 {}({}, {})",
                    expected.join(", "),
                    token.1,
                    token.0,
                    token.2
                )
            }
            ParseResult::ExprParseError(error) => match error {
                tec2_parser::error::Error::CanNotBeAchieved(reason) => {
                    write!(f, "表达式解析错误: {}", reason)
                }
            },
            ParseResult::Result(result) => {
                write!(f, "{}", result)
            }
        }
    }
}

impl Parser {
    fn parse(input: &str) -> ParseResult {
        match grammar::ExprParser::new().parse(input) {
            Ok(expr) => {
                let mut parser = ExprParser::new(expr);
                match parser.parse() {
                    Ok(_) => ParseResult::Result(hex::encode_upper(parser.hex())),
                    Err(error) => ParseResult::ExprParseError(error),
                }
            }
            Err(error) => match error {
                ParseError::InvalidToken { location } => ParseResult::InvalidToken { location },
                ParseError::UnrecognizedEof { location, expected } => {
                    ParseResult::UnrecognizedEof { location, expected }
                }
                ParseError::UnrecognizedToken { token, expected } => {
                    ParseResult::UnrecognizedToken {
                        token: (token.0, token.1.to_string(), token.2),
                        expected,
                    }
                }
                ParseError::ExtraToken { token } => ParseResult::UnrecognizedToken {
                    token: (token.0, token.1.to_string(), token.2),
                    expected: vec![],
                },
                ParseError::User { .. } => unreachable!(),
            },
        }
    }

    fn parse_enter(
        user_input: String,
        result: Arc<Mutex<Vec<String>>>,
        util: AppUtil,
        cursor: usize,
    ) {
        let parse_result = Self::parse(&user_input[USER_INPUT_PREFIX.len()..user_input.len()]);
        loop {
            if let Ok(mut result) = result.try_lock() {
                result[cursor] = parse_result.to_string();
                util.update();
                break;
            }
        }
    }

    fn parse_backend(
        user_input: Arc<Mutex<String>>,
        result: Arc<Mutex<Option<ParseResult>>>,
        util: AppUtil,
        run_flag: Arc<AtomicBool>,
    ) {
        let mut pre_user_input = String::new();
        while run_flag.load(Ordering::Relaxed) {
            if let Ok(user_input) = user_input.try_lock() {
                if pre_user_input != *user_input {
                    pre_user_input = user_input.clone();
                    let parse_result =
                        Self::parse(&user_input[USER_INPUT_PREFIX.len()..user_input.len()]);
                    if let Ok(mut result) = result.try_lock() {
                        *result = Some(parse_result);
                        util.update();
                    }
                }
            }
            std::thread::sleep(Duration::from_millis(500));
        }
    }
}
impl Page for Parser {
    fn new(util: AppUtil) -> Self
    where
        Self: Sized,
    {
        let user_input = Arc::new(Mutex::new(USER_INPUT_PREFIX.to_string()));
        let last_parse_result = Arc::new(Mutex::new(None));
        let run_flag = Arc::new(AtomicBool::new(true));
        let (user_input_, last_parse_result_, util_, run_flag_) = (
            user_input.clone(),
            last_parse_result.clone(),
            util.clone(),
            run_flag.clone(),
        );

        std::thread::spawn(move || {
            Self::parse_backend(user_input_, last_parse_result_, util_, run_flag_)
        });

        Self {
            util,
            cursor: (USER_INPUT_PREFIX.len(), 0),
            max_cursor: 0,
            user_input,
            controller_history: Arc::new(Mutex::new(Vec::new())),
            last_parse_result,
            run_flag,
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        let user_input = self.user_input.lock().unwrap();
        let controller_history = self.controller_history.lock().unwrap();
        let [up_area, message_area] =
            Layout::vertical([Constraint::Fill(2), Constraint::Min(8)]).areas(frame.area());
        let [help_area, controller_area] =
            Layout::horizontal([Constraint::Min(8), Constraint::Fill(4)]).areas(up_area);
        let help = Paragraph::new("1. 不要输入中文，输入中文会导致崩溃\n2. 按<ESC>返回主页")
            .block(
                Block::bordered()
                    .title("帮助")
                    .title_alignment(Alignment::Center),
            )
            .wrap(Wrap { trim: true });
        self.max_cursor = (controller_area.y + controller_area.height - 3) as usize;
        let controller_history_len = controller_history.len();
        let controller = List::new({
            let begin = {
                if self.max_cursor > controller_history_len {
                    0
                } else {
                    controller_history_len - self.max_cursor
                }
            };
            let mut list = controller_history[begin..controller_history_len]
                .iter()
                .map(|item| ListItem::new(item.as_str()))
                .collect::<Vec<_>>();
            list.push(ListItem::new(Text::from(user_input.as_str())));
            list
        })
        .block(
            Block::bordered()
                .title("控制台")
                .title_alignment(Alignment::Center),
        );
        let message = List::new([ListItem::new(Text::from({
            let last_parse_result = self.last_parse_result.lock().unwrap();
            match &*last_parse_result {
                None => "".to_string(),
                Some(data) => data.to_string(),
            }
        }))])
        .block(Block::bordered().title("消息"));
        frame.set_cursor_position(Position::new(
            controller_area.x + 1 + self.cursor.0 as u16,
            controller_area.y + 1 + self.cursor.1 as u16,
        ));
        frame.render_widget(help, help_area);
        frame.render_widget(controller, controller_area);
        frame.render_widget(message, message_area);
    }

    fn handle_key_event(&mut self, event: KeyEvent) {
        match event.kind {
            KeyEventKind::Press | KeyEventKind::Repeat => {
                let mut user_input = self.user_input.lock().unwrap();
                match event.code {
                    KeyCode::Backspace | KeyCode::Delete => {
                        if self.cursor.0 > USER_INPUT_PREFIX.len() {
                            user_input.remove(self.cursor.0 - 1);
                            self.cursor.0 -= 1;
                        }
                    }
                    KeyCode::Left => {
                        self.cursor.0 = max(USER_INPUT_PREFIX.len(), self.cursor.0 - 1);
                    }
                    KeyCode::Right => {
                        self.cursor.0 = min(user_input.len(), self.cursor.0 + 1);
                    }
                    KeyCode::Enter => {
                        let mut controller_history = self.controller_history.lock().unwrap();
                        self.cursor.1 = min(self.cursor.1 + 3, self.max_cursor);
                        self.cursor.0 = USER_INPUT_PREFIX.len();
                        controller_history.push(user_input.clone());
                        controller_history.push("result".to_string());
                        controller_history.push("".to_string());
                        *user_input = USER_INPUT_PREFIX.to_string();
                        let user_input = controller_history[controller_history.len() - 3].clone();
                        let (cursor, controller_history, util) = (
                            controller_history.len() - 2,
                            self.controller_history.clone(),
                            self.util.clone(),
                        );
                        std::thread::spawn(move || {
                            Self::parse_enter(user_input, controller_history, util, cursor)
                        });
                    }
                    KeyCode::Char(char) => {
                        user_input.insert(self.cursor.0, char);
                        self.cursor.0 += 1;
                    }
                    KeyCode::Esc => {
                        self.util.back();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

impl Drop for Parser {
    fn drop(&mut self) {
        self.run_flag.store(false, Ordering::Relaxed)
    }
}
