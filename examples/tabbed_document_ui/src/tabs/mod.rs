use std::marker::PhantomData;
use slotmap::{new_key_type, SlotMap};
use slotmap::basic::Iter;
use iced::{widget, Alignment, Element, Length};
use iced::widget::{column, button, horizontal_space, row, scrollable, text};
use iced::widget::scrollable::{Direction, Scrollbar};

new_key_type! {
    /// A key for a tab
    pub struct TabKey;
}

#[derive(Debug, Clone)]
pub enum TabMessage<TKM> {
    ActivateTab(TabKey),
    CloseTab(TabKey),
    TabKindMessage(TabKey, TKM),
}

pub enum TabAction<TKA, TK> {
    TabSelected(TabKey),
    TabClosed(TabKey, TK),
    TabAction(TKA),
}

pub struct Tabs<TK, TKM, TKA> {
    tabs: SlotMap<TabKey, TK>,
    history: Vec<TabKey>,
    active: Option<TabKey>,
    _phantom1: PhantomData<TKM>,
    _phantom2: PhantomData<TKA>,
}

impl<TK, TKM, TKA> Tabs<TK, TKM, TKA> {
    pub fn push(&mut self, tab_kind: TK) -> TabKey {
        let key = self.tabs.insert(tab_kind);
        self.history.push(key);

        key
    }

    pub fn activate(&mut self, key: TabKey) {
        let _previously_active = self.active.replace(key);
        self.history.push(key);
        self.history.dedup();
    }

    pub fn close_all(&mut self) -> Vec<(TabKey, TK)> {
        let closed_tabs: Vec<(TabKey, TK)> = self.tabs.drain().collect();
        let _previously_selected = self.active.take();
        self.history.clear();
        closed_tabs
    }

    pub fn iter(&self) -> Iter<'_, TabKey, TK> {
        self.tabs.iter()
    }

    pub fn get(&self, key: TabKey) -> Option<&TK> {
        self.tabs.get(key)
    }

    pub fn active(&self) -> Option<(TabKey, &TK)> {
        self.active.map(|key|(key, self.tabs.get(key).unwrap()))
    }

    fn close(&mut self, key: TabKey) -> TK {
        // remove the key from the history, so we don't try and switch to it again.
        self.history.retain(|&other_key| other_key != key);
        self.history.dedup();

        if let Some(recent_key) = self.history.pop() {
            self.activate(recent_key);
        } else {
            let _previously_active = self.active.take();
        }

        let closed_tab = self.tabs.remove(key).unwrap();
        closed_tab
    }
}

impl<TK, TKM, TKA> Default for Tabs<TK, TKM, TKA> {
    fn default() -> Self {
        Self {
            tabs: SlotMap::default(),
            history: Default::default(),
            active: None,
            _phantom1: Default::default(),
            _phantom2: Default::default(),
        }
    }
}

impl<TK, TKM: Clone, TKA> Tabs<TK, TKM, TKA> {
    pub fn update<UF>(
        &mut self,
        message: TabMessage<TKM>,
        mut update_fn: UF
    ) -> TabAction<TKA, TK>
    where
        UF: FnMut(&mut TK, TKM) -> TKA
    {
        match message {
            TabMessage::TabKindMessage(key, message) => {
                let tab = self.tabs.get_mut(key).unwrap();
                let action = update_fn(tab, message);

                TabAction::TabAction(action)
            },
            TabMessage::ActivateTab(key) => {
                self.active = Some(key);
                TabAction::TabSelected(key)
            },
            TabMessage::CloseTab(key) => {
                let closed_tab = self.close(key);
                TabAction::TabClosed(key, closed_tab)
            },
        }
    }

    fn build_tab_bar<'tk, 'label, LF>(&'label self, label_fn: LF) -> Element<'label, TabMessage<TKM>>
    where
        LF: Fn(TabKey, &'label TK) -> String,
    {
        let tab_buttons = self.tabs.iter().map(|(key, tab)|{
            let label = label_fn(key, tab);
            let button = button(
                row![
                    text(label),
                    button("X")
                        .style(button::text)
                        .on_press_with(move || TabMessage::CloseTab(key))
                ]
                    .spacing(4)
                    .align_y(Alignment::Center)
            )
                .on_press_with(move || TabMessage::ActivateTab(key))
                .style(if self.is_active(key) {
                    button::primary
                } else {
                    button::secondary
                })
                .into();

            button
        });

        let tab_bar = scrollable(
            row(tab_buttons)
                .spacing(2)
        )
            .width(Length::Fill)
            .direction(Direction::Horizontal(
                Scrollbar::new().width(2).spacing(0).scroller_width(2),
            ))
            .into();

        tab_bar
    }

    fn is_active(&self, key: TabKey) -> bool {
        match self.active {
            Some(other_key) if key.eq(&other_key) => true,
            _ => false,
        }
    }

    pub fn view<'tk, VF, LF>(
        &'tk self,
        view_fn: VF,
        label_fn: LF,
    ) -> Element<'tk, TabMessage<TKM>>
    where
        VF: Fn(TabKey, &'tk TK) -> Element<'tk, TKM>,
        LF: Fn(TabKey, &TK) -> String,
    {

        let tab_bar = self.build_tab_bar(label_fn);

        let current_tab = if let Some((key, tab)) = self.active() {
            view_fn(key, tab)
                .map(move |message|TabMessage::TabKindMessage(key, message))
        } else {
            horizontal_space().into()
        };


        let content = widget::container(current_tab)
            .width(Length::Fill)
            .height(Length::Fill);
            // .align_x(Alignment::Center)
            // .align_y(Alignment::Center);

        column![tab_bar, content].into()
    }
}
