use std::marker::PhantomData;
use std::ops::RangeFull;
use indexmap::IndexMap;
use indexmap::map::Iter;
use iced::{widget, Alignment, Color, Element, Length, Theme};
use iced::theme::palette::Extended;
use iced::widget::{column, button, horizontal_space, row, scrollable, text, container, vertical_space, horizontal_rule, vertical_rule, Space};
use iced::widget::scrollable::{Direction, Scrollbar};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct TabKey(u64);

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
    next_key_value: u64,
    tabs: IndexMap<TabKey, TK>,
    history: Vec<TabKey>,
    active: Option<TabKey>,
    _phantom1: PhantomData<TKM>,
    _phantom2: PhantomData<TKA>,
}

impl<TK, TKM, TKA> Tabs<TK, TKM, TKA> {

    fn next_key(&mut self) -> TabKey {
        let key = TabKey(self.next_key_value);
        self.next_key_value += 1;

        key
    }

    pub fn push(&mut self, tab_kind: TK) -> TabKey {
        let key = self.next_key();
        let _old_value = self.tabs.insert(key, tab_kind);
        self.history.push(key);

        key
    }

    pub fn replace(&mut self, key: TabKey, tab_kind: TK) {
        if let Some(value) = self.tabs.get_mut(&key) {
            *value = tab_kind
        }
    }

    pub fn activate(&mut self, key: TabKey) {
        let _previously_active = self.active.replace(key);
        self.history.push(key);
        self.history.dedup();
    }

    pub fn close_all(&mut self) -> Vec<(TabKey, TK)> {
        let closed_tabs: Vec<(TabKey, TK)> = self.tabs.drain(RangeFull).collect();
        let _previously_selected = self.active.take();
        self.history.clear();
        closed_tabs
    }

    pub fn iter(&self) -> Iter<'_, TabKey, TK> {
        self.tabs.iter()
    }

    pub fn get(&self, key: TabKey) -> Option<&TK> {
        self.tabs.get(&key)
    }

    pub fn active(&self) -> Option<(TabKey, &TK)> {
        self.active.map(|key|(key, self.tabs.get(&key).unwrap()))
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

        let closed_tab = self.tabs.shift_remove(&key).unwrap();
        closed_tab
    }
}

impl<TK, TKM, TKA> Default for Tabs<TK, TKM, TKA> {
    fn default() -> Self {
        Self {
            next_key_value: 0,
            tabs: IndexMap::default(),
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
        UF: FnMut(TabKey, &mut TK, TKM) -> TKA
    {
        match message {
            TabMessage::TabKindMessage(key, message) => {
                let tab = self.tabs.get_mut(&key).unwrap();
                let action = update_fn(key, tab, message);

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
            let label = label_fn(*key, tab);
            let button = button(
                column![
                    row![
                        text(label),
                        button("X")
                            .style(button::text)
                            .on_press_with(move || TabMessage::CloseTab(*key)),
                    ]
                        .spacing(4)
                        .align_y(Alignment::Center),
                    // FIXME this doesn't get displayed, if you add a `.width(Length::Fill)` it panics
                    /*
                    container(
                        Space::with_height(4)
                    )
                        .style(|theme: &Theme|{
                            let palette: Extended = *theme.extended_palette();

                            if self.is_active(*key) {
                                container::background(palette.primary.base.color)
                            } else {
                                container::background(palette.secondary.base.color)
                            }
                        })
                     */
                ]
            )
                .on_press_with(move || TabMessage::ActivateTab(*key))
                .style(if self.is_active(*key) {
                    button::primary
                } else {
                    button::secondary
                })
                // FIXME when the above is fixed, always use secondary styling and use box to indicate active tab
                //.style(button::secondary)
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
