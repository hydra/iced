use std::marker::PhantomData;
use iced_aw::style::tab_bar;
use iced_aw::TabLabel;
use slotmap::{new_key_type, SlotMap};
use slotmap::basic::Iter;
use iced::{Element, Length};

new_key_type! {
    /// A key for a tab
    pub struct TabKey;
}

#[derive(Debug, Clone)]
pub enum TabMessage<TKM> {
    SelectTab(TabKey),
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
    selected: Option<TabKey>,
    _phantom1: PhantomData<TKM>,
    _phantom2: PhantomData<TKA>,
}

impl<TK, TKM, TKA> Tabs<TK, TKM, TKA> {
    pub fn push(&mut self, tab_kind: TK) -> TabKey {
        self.tabs.insert(tab_kind)
    }

    pub fn activate(&mut self, key: TabKey) {
        let _previously_selected = self.selected.replace(key);
    }

    pub fn close_all(&mut self) -> Vec<(TabKey, TK)> {
        let closed_tabs: Vec<(TabKey, TK)> = self.tabs.drain().collect();
        let _previously_selected = self.selected.take();
        closed_tabs
    }

    pub fn iter(&self) -> Iter<'_, TabKey, TK> {
        self.tabs.iter()
    }

    pub fn get(&self, key: TabKey) -> Option<&TK> {
        self.tabs.get(key)
    }
}

impl<TK, TKM, TKA> Default for Tabs<TK, TKM, TKA> {
    fn default() -> Self {
        Self {
            tabs: SlotMap::default(),
            selected: None,
            _phantom1: Default::default(),
            _phantom2: Default::default(),
        }
    }
}

impl<TK, TKM, TKA> Tabs<TK, TKM, TKA> {
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
            TabMessage::SelectTab(key) => {
                self.selected = Some(key);
                TabAction::TabSelected(key)
            },
            TabMessage::CloseTab(key) => {
                match self.selected {
                    Some(selected) if selected == key => {
                        let _previously_selected = self.selected.take();
                    }
                    _ => {}
                }
                let closed_tab = self.tabs.remove(key).unwrap();
                TabAction::TabClosed(key, closed_tab)
            },
        }
    }

    pub fn view<'tk, VF, LF>(
        &'tk self,
        view_fn: VF,
        label_fn: LF,
    ) -> Element<'tk, TabMessage<TKM>>
    where
        VF: Fn(TabKey, &'tk TK) -> Element<'tk, TKM>,
        LF: Fn(TabKey, &'tk TK) -> String,
    {
        let tab_bar = self.tabs
            .iter()
            .fold(
                iced_aw::Tabs::<TabMessage<TKM>, TabKey>::new(|tab_key|{
                    TabMessage::SelectTab(tab_key)
                })
                    .tab_icon_position(iced_aw::tabs::Position::Bottom)
                    .on_close(|tab_key|{
                        TabMessage::CloseTab(tab_key)
                    })
                    .tab_bar_style(Box::new(tab_bar::primary))
                ,
                |tab_bar, (key, tab)| {

                    // FIXME actions in ANY tabs, e.g. text selection in a texttab, cause ALL tab views to be re-created!
                    let tab_view = view_fn(key, tab);

                    let view = tab_view
                        .map(move |message|{
                            TabMessage::TabKindMessage(key, message)
                        });

                    let label = label_fn(key, tab);

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
            .height(Length::Fill)
            .into();

        tab_bar
    }
}
