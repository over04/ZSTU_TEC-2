use super::{Route, Router, RouterManager};
use ratatui::crossterm::event;
use ratatui::crossterm::event::Event;
use ratatui::{DefaultTerminal, Frame};
use std::sync::{Arc, Condvar, Mutex, RwLock};

type Lock = Arc<(Mutex<bool>, Condvar)>;
pub struct AppEnv {
    running: bool,
    flag: Option<AppUtilFlag>,
}

pub enum AppUtilFlag {
    Back,
    Quit,
    Push(Route),
    Event(Event),
}

pub struct AppUtil {
    lock: Lock,
    env: Arc<RwLock<Box<AppEnv>>>,
}

impl AppUtil {
    pub fn quit(&self) {
        self.env.write().unwrap().flag = Some(AppUtilFlag::Quit);
        self.update();
    }

    pub fn push_route(&self, route: Route) {
        self.env.write().unwrap().flag = Some(AppUtilFlag::Push(route));
        self.update();
    }

    pub fn back(&self) {
        self.env.write().unwrap().flag = Some(AppUtilFlag::Back);
        self.update();
    }

    pub fn update(&self) {
        let (lock, cvar) = &*self.lock; // 获取内部引用，相当于lock.as_ref()
        lock.lock().map(|mut lock| *lock = true).ok();
        cvar.notify_one();
    }
}

pub struct App {
    lock: Lock,
    env: Arc<RwLock<Box<AppEnv>>>,
    router_manager: RouterManager,
    // util: Arc<AppUtil>,
}

impl App {
    pub fn new<T: Router>() -> Self {
        let env = Arc::new(RwLock::new(Box::new(AppEnv {
            running: false,
            flag: None,
        })));
        let lock = Lock::new((Mutex::new(false), Condvar::new()));
        let util = Arc::new(AppUtil {
            lock: lock.clone(),
            env: env.clone(),
        });
        let app = Self {
            lock: lock.clone(),
            env: env.clone(),
            // util: util.clone(),
            router_manager: RouterManager::new::<T>(util.clone()),
        };

        std::thread::spawn(|| Self::handle_crossterm_events_background(env, lock));
        app
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        self.env.write().unwrap().running = true;
        while self.env.read().unwrap().running {
            let data = self.router_manager.pop();
            if let Some(mut page) = data {
                self.handle_draw(&mut terminal, &mut page)?;
                while self.env.read().unwrap().running {
                    {
                        let (lock, cvar) = &*self.lock;
                        let mut guard = lock.lock().unwrap();
                        match *guard {
                            true => {
                                *guard = false;
                            }
                            false => {
                                *guard = true;
                                guard = cvar.wait(guard).unwrap();
                                *guard = false;
                            }
                        }
                    }
                    let flag = self.env.write().unwrap().flag.take();
                    if let Some(flag) = flag {
                        match flag {
                            AppUtilFlag::Back => {
                                break;
                            }
                            AppUtilFlag::Quit => {
                                self.env.write().unwrap().running = false;
                                break;
                            }
                            AppUtilFlag::Push(route) => {
                                self.router_manager.push(page);
                                self.router_manager.push(route);
                                break;
                            }
                            AppUtilFlag::Event(event) => {
                                self.handle_crossterm_events(event, &mut page);
                            }
                        }
                    }
                    self.handle_draw(&mut terminal, &mut page)?;
                }
            } else {
                break;
            }
        }
        Ok(())
    }

    fn handle_crossterm_events_background(
        env: Arc<RwLock<Box<AppEnv>>>,
        lock: Lock,
    ) -> color_eyre::Result<()> {
        loop {
            let event = event::read()?;
            env.write().unwrap().flag = Some(AppUtilFlag::Event(event));
            let (lock, cvar) = &*lock; // 获取内部引用，相当于lock.as_ref()
            {
                let _guard = lock.lock().unwrap();
            }
            cvar.notify_one();
        }
    }

    fn handle_draw(
        &mut self,
        terminal: &mut DefaultTerminal,
        page: &mut Route,
    ) -> color_eyre::Result<()> {
        terminal.draw(|frame: &mut Frame| page.render(frame))?;
        Ok(())
    }

    fn handle_crossterm_events(&mut self, event: Event, page: &mut Route) {
        match event {
            Event::Key(event) => page.handle_key_event(event),
            _ => {}
        }
    }
}
