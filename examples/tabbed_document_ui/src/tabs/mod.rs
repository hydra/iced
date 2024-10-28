use std::marker::PhantomData;
use iced_aw::style::tab_bar;
use iced_aw::TabLabel;
use slotmap::{new_key_type, SlotMap};
use iced::Element;

new_key_type! {
    /// A key for a tab
    pub struct TabKey;
}

#[derive(Debug, Clone)]
pub enum TabMessage<TKM> {
    TabSelected(TabKey),
    TabClosed(TabKey),
    TabKindMessage(TabKey, TKM),
}

pub enum TabAction<TKA> {
    TabSelected(TabKey),
    TabClosed(TabKey),
    TabAction(TKA),
}

/// Individual re-usable tabs should implement this
pub trait Tab {
    type Message;
    type Action;

    fn view(&self) -> Element<'static, Self::Message>;
    fn label(&self) -> String;

    fn update(&mut self, message: Self::Message) -> Self::Action;
}

/// The application, which uses re-usable tabs, should implement this
pub trait AppTabs<TKM, TKA> {
    fn view<'a>(&self, key: TabKey) -> Element<'a, TKM>;
    fn label(&self, key: TabKey) -> String;
    fn update(&mut self, message: TKM) -> TKA;
}

pub struct Tabs<TK: AppTabs<TKM, TKA>, TKM, TKA> {
    tabs: SlotMap<TabKey, TK>,
    selected: Option<TabKey>,
    _phantom1: PhantomData<TKM>,
    _phantom2: PhantomData<TKA>,
}

impl<TK: AppTabs<TKM, TKA>, TKM, TKA> Tabs<TK, TKM, TKA> {
    pub fn push(&mut self, tab_kind: TK) -> TabKey {
        self.tabs.insert(tab_kind)
    }

    pub fn close_all(&mut self) {
        // FIXME this should probably generate multiple TabAction::TabClosed actions instead of this
        let _previously_selected = self.selected.take();
        self.tabs.clear()
    }
}

impl<TK: AppTabs<TKM, TKA>, TKM, TKA> Default for Tabs<TK, TKM, TKA> {
    fn default() -> Self {
        Self {
            tabs: SlotMap::default(),
            selected: None,
            _phantom1: Default::default(),
            _phantom2: Default::default(),
        }
    }
}

impl<TK: AppTabs<TKM, TKA>, TKM, TKA> Tabs<TK, TKM, TKA> {
    pub fn update(
        &mut self, message: TabMessage<TKM>
    ) -> TabAction<TKA> {
        match message {
            TabMessage::TabKindMessage(key, message) => {
                let tab = self.tabs.get_mut(key).unwrap();
                let action = tab.update(message);

                TabAction::TabAction(action)
            },
            TabMessage::TabSelected(key) => {
                self.selected = Some(key);
                TabAction::TabSelected(key)
            },
            TabMessage::TabClosed(key) => {
                match self.selected {
                    Some(selected) if selected == key => {
                        let _previously_selected = self.selected.take();
                    }
                    _ => {}
                }
                let _closed_tab = self.tabs.remove(key).unwrap();
                TabAction::TabClosed(key)
            },
        }
    }

    pub fn view<'tk>(
        &'tk self
    ) -> Element<'tk, TabMessage<TKM>> {
        let tab_bar = self.tabs
            .iter()
            .fold(
                iced_aw::Tabs::<TabMessage<TKM>, TabKey>::new(|tab_key|{
                    TabMessage::TabSelected(tab_key)
                })
                    .tab_icon_position(iced_aw::tabs::Position::Bottom)
                    .on_close(|tab_key|{
                        TabMessage::TabClosed(tab_key)
                    })
                    .tab_bar_style(Box::new(tab_bar::primary))
                ,
                |tab_bar, (key, tab)| {

                    let tab_view = tab.view(key);

                    let view = tab_view
                        .map(move |message|{
                            TabMessage::TabKindMessage(key, message)
                        });

                    let label = tab.label(key);

                    let tab_bar = tab_bar.push(key, TabLabel::Text(label), view);

                    match self.selected {
                        Some(selected_key) if selected_key == key => {
                            tab_bar.set_active_tab(&selected_key)
                        }
                        _ => tab_bar
                    }
                }
            );

        let tab_bar: Element<'tk, TabMessage<TKM>> = tab_bar
            .into();

        tab_bar
    }
}
