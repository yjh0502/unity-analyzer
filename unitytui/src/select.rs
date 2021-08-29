use super::*;
use std::collections::HashSet;
use tui::backend::Backend;
use tui::Frame;

#[derive(Clone)]
pub struct StateReverseRef {
    list: ListState,
    items: Vec<(String, String)>,
}

impl InitializedState {
    pub fn state_reverse_ref(&self, dst_guid: &str) -> StateReverseRef {
        let idx = &self.index;

        let refs = idx.backward_refs(&dst_guid);
        let mut guids = HashSet::new();

        let mut items = Vec::new();
        for r in refs {
            let src_guid = &r.src_guid;

            if guids.contains(src_guid.as_str()) {
                continue;
            }
            guids.insert(src_guid.to_owned());

            let path = idx.try_asset_path_by_guid(src_guid);
            items.push((format!("{:?}", path), src_guid.to_owned()));
        }

        StateReverseRef {
            list: ListState::default(),
            items,
        }
    }

    #[allow(unused)]
    pub(crate) fn render_reverse_ref<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        rect: Rect,
        state: &mut StateReverseRef,
    ) {
        let items = state
            .items
            .iter()
            .map(|(path, _guid)| {
                let span = Span::from(path.to_owned());
                ListItem::new(Spans::from(vec![span]))
            })
            .collect::<Vec<_>>();

        let items = List::new(items)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");

        f.render_stateful_widget(items, rect, &mut state.list);
    }
}
