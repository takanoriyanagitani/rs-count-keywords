use std::collections::BTreeMap;

pub struct PatternStat {
    pub id2pattern: BTreeMap<u32, String>,
    pub stat: BTreeMap<u32, u32>,
}
