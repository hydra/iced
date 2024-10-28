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

pub trait Tab {
    type Message;
    type Action;

    fn view(&self) -> Element<'static, Self::Message>;
    fn label(&self) -> String;

    fn update(&mut self, message: Self::Message) -> Self::Action;
}

pub struct Tabs<TK, TKM, TKA> {
    tabs: SlotMap<TabKey, TK>,
    _phantom1: PhantomData<TKM>,
    _phantom2: PhantomData<TKA>,
}

impl<TK, TKM, TKA> Tabs<TK, TKM, TKA> {
    pub fn push(&mut self, tab_kind: TK) -> TabKey {
        self.tabs.insert(tab_kind)
    }
}

impl<TK, TKM, TKA> Default for Tabs<TK, TKM, TKA> {
    fn default() -> Self {
        Self {
            tabs: SlotMap::default(),
            _phantom1: Default::default(),
            _phantom2: Default::default(),
        }
    }
}

impl<TK, TKM, TKA> Tabs<TK, TKM, TKA> {
    pub fn update(&mut self, message: TabMessage<TKM>, update_fn: &dyn Fn(&mut TK, TKM) -> TKA) -> TabAction<TKA> {
        match message {
            TabMessage::TabKindMessage(key, message) => {
                let tab = self.tabs.get_mut(key).unwrap();
                let action = update_fn(tab, message);

                TabAction::TabAction(action)
            },
            TabMessage::TabSelected(key) => TabAction::TabSelected(key),
            TabMessage::TabClosed(key) => TabAction::TabClosed(key),
        }
    }

    pub fn view<'tk>(
        &'tk self,
        view_fn: &dyn Fn(&'tk TK, TabKey) -> Element<'_, TKM>,
        label_fn: & dyn Fn(&'tk TK, TabKey) -> String,
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

                    let tab_view = view_fn(tab, key);

                    let view = tab_view
                        .map(move |message|{
                            TabMessage::TabKindMessage(key, message)
                        });

                    let label = label_fn(tab, key);

                    tab_bar.push(key, TabLabel::Text(label), view)
                }
            );

        let tab_bar: Element<'tk, TabMessage<TKM>> = tab_bar
            .into();

        tab_bar
    }
}