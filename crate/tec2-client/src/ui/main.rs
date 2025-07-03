use crate::app::{AppUtil, Page};
use crate::ui::Parser;
use figlet_rs::FIGfont;
use num_enum::TryFromPrimitive;
use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::{Constraint, Layout, Line, Stylize};
use ratatui::style::Modifier;
use ratatui::widgets::Paragraph;
use std::cmp::{max, min};

#[derive(PartialEq, TryFromPrimitive, Clone)]
#[repr(u8)]
enum Menu {
    Parser = 1,
    Config = 2,
    Exit = 3,
}

impl Menu {
    fn as_str(&self) -> &'static str {
        match self {
            Menu::Parser => "解释器",
            Menu::Config => "设置",
            Menu::Exit => "退出",
        }
    }
}

pub struct Main {
    menu: Menu,
    util: AppUtil,
}
impl Page for Main {
    fn new(util: AppUtil) -> Self
    where
        Self: Sized,
    {
        Self {
            menu: Menu::Parser,
            util,
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        let [_, title_area, _, menu_area, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(5),
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .areas(frame.area());
        let title = Paragraph::new(
            FIGfont::standard()
                .unwrap()
                .convert("TEC-2 TOOL")
                .unwrap()
                .to_string(),
        )
        .centered();
        let menu = {
            let mut menu = vec![" ".into()];
            for i in [Menu::Parser, Menu::Config, Menu::Exit].iter() {
                if &self.menu == i {
                    menu.push(i.as_str().add_modifier(Modifier::REVERSED));
                } else {
                    menu.push(i.as_str().into());
                }
                menu.push("   ".into());
            }
            Line::from(menu).centered()
        };
        frame.render_widget(title, title_area);
        frame.render_widget(menu, menu_area);
    }

    fn handle_key_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('q') => {
                self.util.back();
            }
            KeyCode::Char('a') | KeyCode::Left => {
                self.menu =
                    Menu::try_from(max(Menu::Parser as u8, self.menu.clone() as u8 - 1)).unwrap();
            }
            KeyCode::Char('d') | KeyCode::Right => {
                self.menu =
                    Menu::try_from(min(Menu::Exit as u8, self.menu.clone() as u8 + 1)).unwrap();
            }
            KeyCode::Enter => match self.menu {
                Menu::Parser => self
                    .util
                    .push_route(Box::new(Parser::new(self.util.clone()))),
                Menu::Config => {}
                Menu::Exit => self.util.quit(),
            },
            _ => {}
        };
    }
}
