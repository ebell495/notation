use fehler::throws;
use notation_proto::prelude::ProtoEntry;
use std::collections::HashMap;
use std::convert::TryFrom;

use std::sync::{Arc, Weak};
use thiserror::Error;

use crate::prelude::{Form, ModelEntry, Section, Tab, TabBar, TabMeta, Track};

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("track not found")]
    TrackNotFound(String),
    #[error("section not found")]
    SectionNotFound(String),
}

impl Tab {
    #[throws(ParseError)]
    pub fn try_parse_arc(v: notation_proto::prelude::Tab) -> Arc<Self> {
        let meta = Arc::new(v.meta);
        let tracks = v.tracks.into_iter().map(|x| Arc::new(x.into())).collect();
        let mut sections = Vec::new();
        for section in v.sections {
            sections.push(Section::try_from((section, &tracks)).map(Arc::new)?);
        }
        let form = Form::try_from((v.form, &sections))?;
        Self::new_arc(meta, tracks, sections, form)
    }
}
impl Tab {
    pub fn new_arc(
        meta: Arc<TabMeta>,
        tracks: Vec<Arc<Track>>,
        sections: Vec<Arc<Section>>,
        form: Form,
    ) -> Arc<Self> {
        Arc::<Tab>::new_cyclic(|weak_self| {
            let bars = Self::new_tab_bars(weak_self, &form);
            Self {
                meta,
                tracks,
                sections,
                form,
                bars,
            }
        })
    }
    fn new_tab_bars(weak_self: &Weak<Tab>, form: &Form) -> Vec<Arc<TabBar>> {
        let mut section_rounds: HashMap<String, usize> = HashMap::new();
        let mut section_ordinal: usize = 1;
        let mut bar_ordinal: usize = 1;
        let mut bars: Vec<Arc<TabBar>> = vec![];
        for section in form.sections.iter() {
            let section_round = match section_rounds.get(&section.id) {
                Some(r) => r + 1,
                None => 1,
            };
            section_rounds.insert(section.id.clone(), section_round);
            bars.extend(section.new_tab_bars(
                section.clone(),
                weak_self.clone(),
                section_round,
                section_ordinal,
                bar_ordinal,
            ));
            section_ordinal += 1;
            bar_ordinal += section.bars.len();
        }
        println!("new_tab_bars() -> {:?} bars", bars.len());
        bars
    }
}
impl Section {
    pub fn new_tab_bars(
        &self,
        arc_section: Arc<Section>,
        tab: Weak<Tab>,
        section_round: usize,
        section_ordinal: usize,
        section_bar_ordinal: usize,
    ) -> Vec<Arc<TabBar>> {
        self.bars
            .iter()
            .enumerate()
            .map(|(bar_index, bar)| TabBar {
                tab: tab.clone(),
                section: arc_section.clone(),
                section_round,
                section_ordinal,
                bar: bar.clone(),
                bar_index,
                bar_ordinal: section_bar_ordinal + bar_index,
            })
            .map(Arc::new)
            .collect()
    }
}
impl ModelEntry {
    pub fn new_entries(v: Vec<ProtoEntry>) -> Vec<Arc<ModelEntry>> {
        let _entries: Vec<Arc<ProtoEntry>> =
            v.into_iter()
            .map(Arc::new)
            .collect();
        let entries = Arc::new(_entries.clone());
        _entries.into_iter().enumerate()
            .map(|(index, entry)|
                ModelEntry::new(entries.clone(), index, entry)
            ).map(Arc::new)
            .collect()
    }
}
