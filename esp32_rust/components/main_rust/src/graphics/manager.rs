use std::collections::HashMap;
use crate::graphics::drivers::GraphicsDriver;
use crate::graphics::event::GraphicsEvent;
use crate::graphics::screen::Screen;

pub struct GraphicsManager<'a> {
    drivers: Vec<Box<dyn GraphicsDriver>>,
    screens_factories: HashMap<&'a str, Box<dyn Fn() -> Box<dyn Screen>>>,
    current_screen: Option<(&'a str, Box<dyn Screen>)>,
    backstack: Vec<&'a str>
}

impl<'a> GraphicsManager<'a> {
    pub fn new() -> GraphicsManager<'a> {
        GraphicsManager {
            drivers: vec![],
            screens_factories: HashMap::new(),
            current_screen: None,
            backstack: vec![]
        }
    }

    pub fn load_screen(&mut self, id: &'a str, factory: Box<dyn Fn() -> Box<dyn Screen + 'static>>) {
        self.screens_factories.insert(id, factory);
    }

    fn navigate_screen(&mut self, id: &'a str, mut screen: Box<dyn Screen>) {
        if let Some(current_screen) = self.current_screen.as_mut() {
            current_screen.1.on_end();
        }

        screen.on_start();
        self.current_screen = Some((id, screen));
    }

    pub fn navigate(&mut self, id: &str) {
        let (id, factory) = self.screens_factories.get_key_value(id).expect("Couldn't find screen");

        if let Some(current_screen) = self.current_screen.as_mut() {
            self.backstack.push(current_screen.0);
        }

        self.navigate_screen(id, factory());
    }
    pub fn navigate_back(&mut self) {
        if let Some(id) = self.backstack.pop() {
            let (id, factory) = self.screens_factories.get_key_value(id).expect("Couldn't find screen");
            self.navigate_screen(id, factory());
        } else {
            println!("Backstack is empty");
        }
    }

    pub fn install_driver(&mut self, mut driver: Box<dyn GraphicsDriver>) {
        driver.init();
        self.drivers.push(driver);
    }

    pub fn update(&mut self) {
        let (_, screen) = self.current_screen.as_ref().expect("Screen is null");
        let rows = screen.render();

        for driver in self.drivers.iter_mut() {
            driver.clear();
            driver.draw(&rows);
        }
    }

    pub fn send_event(&mut self, event: GraphicsEvent) {
        if let Some((_, screen)) = &mut self.current_screen {
            match event {
                GraphicsEvent::Click => {
                    screen.on_click();
                }
                GraphicsEvent::Back => {
                    if !screen.on_back() {
                        self.navigate_back();
                    }
                }
                GraphicsEvent::ScrollLeft => {
                    screen.on_scroll(false);
                }
                GraphicsEvent::ScrollRight => {
                    screen.on_scroll(true);
                }
                _ => {
                    println!("Error: unsupported event type for send_event ({:?})", event);
                }
            }
        } else {
            println!("Current screen is null");
        }
    }

    pub fn send_custom_event(&mut self, event: u32) {
        let (_, screen) = self.current_screen.as_mut().expect("Current screen is null");
        screen.on_custom_event(event);
    }
}
