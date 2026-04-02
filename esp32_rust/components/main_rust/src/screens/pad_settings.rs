use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::graphics::event::GraphicsEvent;
use crate::graphics::screen::{Screen, ScreenData};
use crate::graphics::ui::text::UIText;

pub struct PadSettings {
    data: ScreenData,
    pads_manager_paused: Arc<AtomicBool>
}

impl PadSettings {
    pub fn factory(
        pads_manager_paused: Arc<AtomicBool>
    ) -> impl Fn() -> Box<dyn Screen> + 'static {
        move || {
            Box::new(Self::new(pads_manager_paused.clone()))
        }
    }

    pub fn new(
        pads_manager_paused: Arc<AtomicBool>
    ) -> Self {
        PadSettings {
            data: ScreenData::new(),
            pads_manager_paused
        }
    }

    fn select_pad(&mut self) {
        self.data.clear();

        self.data.add_element(UIText::new("Press button".to_string()));
    }
}

impl Screen for PadSettings {
    fn get_data(&self) -> &ScreenData {
        &self.data
    }
    fn get_data_mut(&mut self) -> &mut ScreenData {
        &mut self.data
    }

    fn on_start(&mut self) {
        self.data.row_offset = 0;
        for elem in self.data.elements.iter_mut() {
            elem.on_event(GraphicsEvent::ScreenStart);
        }

        self.pads_manager_paused.store(true, Ordering::Relaxed);

        self.select_pad();
    }

    fn on_end(&mut self) {
        for elem in self.get_data_mut().elements.iter_mut() {
            elem.on_event(GraphicsEvent::ScreenEnd);
        }

        self.pads_manager_paused.store(false, Ordering::Relaxed);
    }

    fn on_custom_event(&mut self, event: u32) -> bool {
        let channel: u8 = (event & 0b111) as u8;
        let long_press = (event & 0b1000) > 0;

        log::info!("Custom event: {} {}", long_press, channel);

        false
    }
}