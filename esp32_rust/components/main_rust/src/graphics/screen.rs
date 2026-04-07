use crate::graphics::event::GraphicsEvent;
use crate::graphics::ui::element::{UIElement, UIElementState};
use crate::graphics::ui::text::UIText;

pub struct ScreenData {
    pub elements: Vec<Box<dyn UIElement>>,
    pub row_offset: usize,
    pub focus: bool
}

impl ScreenData {
    pub fn new() -> ScreenData {
        ScreenData {
            elements: Vec::new(),
            row_offset: 0,
            focus: false
        }
    }

    pub fn add_element<T>(&mut self, item: T) where T: UIElement + 'static {
        self.elements.push(Box::new(item));
    }

    pub fn add_text(&mut self, text: String) {
        self.elements.push(Box::new(UIText::new(text)));
    }

    pub fn clear(&mut self) {
        self.elements.clear();
    }
}

pub trait Screen {
    fn get_data(&self) -> &ScreenData;
    fn get_data_mut(&mut self) -> &mut ScreenData;

    fn on_start(&mut self) {
        let data = self.get_data_mut();
        data.row_offset = 0;
        for elem in data.elements.iter_mut() {
            elem.on_event(GraphicsEvent::ScreenStart);
        }
    }
    fn on_end(&mut self) {
        for elem in self.get_data_mut().elements.iter_mut() {
            elem.on_event(GraphicsEvent::ScreenEnd);
        }
    }

    fn on_click(&mut self) {
        let data = self.get_data_mut();
        let elem = data.elements.get_mut(data.row_offset).expect("out of range");
        data.focus = elem.on_event(GraphicsEvent::Click);
    }

    fn on_back(&mut self) -> bool {
        let data = self.get_data_mut();
        if data.focus {
            data.focus = false;
            return data.elements[data.row_offset].on_event(GraphicsEvent::Back);
        }
        false
    }

    fn on_scroll(&mut self, direction: bool) {
        let data = self.get_data_mut();
        let row_offset = data.row_offset;
        if data.focus {
            let event = if direction {
                GraphicsEvent::ScrollRight
            } else {
                GraphicsEvent::ScrollLeft
            };
            data.elements[row_offset].on_event(event);
        } else {
            if direction {
                data.row_offset = (row_offset + 1).min(data.elements.len() - 1);
            } else {
                if row_offset == 0 {
                    return;
                }
                data.row_offset -= 1;
            }
        }
    }

    fn on_custom_event(&mut self, _event: u32) -> bool {
        false
    }

    fn refresh(&mut self) {}

    fn render(&self, selector_press: bool) -> Vec<String> {
        let data = self.get_data();
        let i = data.row_offset;
        let len = data.elements.iter().len();
        let end = (i + 2).min(len);

        if i < len {
            data.elements[i..end]
                .iter()
                .enumerate()
                .map(|(index, item)| item.render(
                    if index == 0 {
                        if selector_press {
                            UIElementState::SelectorPress
                        } else {
                            UIElementState::Selected
                        }
                    } else {
                        UIElementState::None
                    }
                ))
                .collect()
        } else {
            vec![]
        }
    }
}
