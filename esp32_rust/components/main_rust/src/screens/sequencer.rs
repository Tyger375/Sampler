use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};
use crate::graphics::event::GraphicsEvent;
use crate::graphics::manager::ScreenArgs;
use crate::graphics::screen::{Screen, ScreenData};
use crate::graphics::ui::button::UIButton;
use crate::graphics::ui::checkbox::UICheckBox;
use crate::graphics::ui::row::UIRow;
use crate::graphics::ui::text::UIText;
use crate::navigator::{Navigator, NavigatorMessage};
use crate::sequencer::Sequencer;
use crate::settings::component::SettingsComponent;
use crate::settings::manager::SettingsManager;
use crate::settings_components::sequencer_projects::SequencerProjects;
use crate::settings_components::SettingsEvent;

pub struct SequencerScreen {
    data: ScreenData,
    navigator: Navigator,
    sequencer: Arc<Sequencer>,
    settings_manager: Arc<SettingsManager<SettingsEvent>>,
    index: Arc<AtomicU8>
}

impl SequencerScreen {
    pub fn factory(
        navigator: Navigator,
        sequencer: Arc<Sequencer>,
        settings_manager: Arc<SettingsManager<SettingsEvent>>
    ) -> impl Fn(ScreenArgs) -> Box<dyn Screen> {
        move |_| Box::new(SequencerScreen::new(navigator.clone(), sequencer.clone(), settings_manager.clone()))
    }

    pub fn new(
        navigator: Navigator,
        sequencer: Arc<Sequencer>,
        settings_manager: Arc<SettingsManager<SettingsEvent>>
    ) -> Self {
        let mut screen = Self {
            data: ScreenData::new(),
            navigator,
            sequencer,
            settings_manager,
            index: Arc::new(AtomicU8::new(0))
        };
        screen.sequencer_options();
        screen
    }

    pub fn sequencer_options(&mut self) {
        self.data.row_offset = 0;
        self.data.clear();

        self.data.add_element(UIText::new("Sequencer".to_string()));

        let sequencer = self.sequencer.clone();
        let navigator = self.navigator.clone();
        let enabled = sequencer.enabled();
        self.data.add_element(UICheckBox::new(
            "Enabled".to_string(),
            move |value| {
                sequencer.set_enable(value);
                navigator.send(NavigatorMessage::graphics_event(GraphicsEvent::Refresh("".to_string()))).ok();
                value
            },
            enabled
        ));

        let sequencer = self.sequencer.clone();
        let navigator = self.navigator.clone();
        if let Some(current_project_name) = sequencer.get_project_name_or_null() {
            let nav = navigator.clone();
            self.data.add_element(UIButton::new(
                format!("Curr: {}", current_project_name),
                move || {
                    nav.send(NavigatorMessage::navigate("sequencer_project")).ok();
                }
            ));
        }

        let mut project_row = UIRow::with_label("Project");

        let sequencer = self.sequencer.clone();
        let navigator = self.navigator.clone();
        let settings = self.settings_manager.clone();
        project_row.add_element(UIButton::new("New".to_string(), move || {
            let names = settings.get_component("sequencer_projects", |component: &SequencerProjects| {
                component.read_projects()
            }).unwrap();
            let mut i = 1;
            let mut name = String::from("Project1");
            while names.contains(&name) {
                i += 1;
                name = format!("Project{i}");
            }
            sequencer.new_project(name);
            navigator.send(NavigatorMessage::navigate("sequencer_project")).ok();
        }));

        let navigator = self.navigator.clone();
        let index = self.index.clone();
        project_row.add_element(UIButton::new("Open".to_string(), move || {
            index.store(1, Ordering::Relaxed);
            navigator.send(NavigatorMessage::graphics_event(GraphicsEvent::refresh("page_change"))).ok();
        }));

        self.data.add_element(project_row);
    }

    pub fn select_project(&mut self) {
        self.data.row_offset = 0;
        self.data.clear();

        self.data.add_text("Select Project".to_string());

        let projects = self.settings_manager.get_component("sequencer_projects", |component: &SequencerProjects| {
            component.read_projects()
        }).unwrap();

        for project in projects.into_iter() {
            let navigator = self.navigator.clone();
            let sequencer = self.sequencer.clone();
            let settings = self.settings_manager.clone();
            self.data.add_element(UIButton::new(
                project.clone(),
                move || {
                    settings.get_component("sequencer_projects", |component: &SequencerProjects| {
                        sequencer.load_project(component.get_project(&project));
                        navigator.send(NavigatorMessage::navigate("sequencer_project")).ok();
                    });
                }
            ));
        }
    }
}

impl Screen for SequencerScreen {
    fn get_data(&self) -> &ScreenData {
        &self.data
    }
    fn get_data_mut(&mut self) -> &mut ScreenData {
        &mut self.data
    }

    fn on_back(&mut self) -> bool {
        let data = self.get_data_mut();
        if data.focus {
            data.focus = false;
            return data.elements[data.row_offset].on_event(GraphicsEvent::Back);
        }

        if self.index.load(Ordering::Relaxed) > 0 {
            self.index.store(0, Ordering::Relaxed);
            self.refresh("page_change".to_string());
            true
        } else {
            false
        }
    }

    fn refresh(&mut self, message: String) {
        match message.as_str() {
            "page_change" => {
                match self.index.load(Ordering::Relaxed) {
                    1 => self.select_project(),
                    _ => self.sequencer_options(),
                };
            },
            _ => ()
        }
    }
}