use std::io;

use std::io::Write;

use crate::stat::patcnt::PatternStat;

#[derive(serde::Serialize)]
pub struct Pair<'a> {
    pub keyword: &'a str,
    pub count: u32,
}

pub fn stat2writer<W>(s: &PatternStat, mut wtr: W) -> Result<(), io::Error>
where
    W: Write,
{
    for id2cnt in &s.stat {
        let (id, cnt) = id2cnt;
        let pat: &str = s
            .id2pattern
            .get(id)
            .ok_or_else(|| io::Error::other(format!("invalid id: {id}")))?;
        let pair = Pair {
            keyword: pat,
            count: *cnt,
        };
        serde_json::to_writer(&mut wtr, &pair)?;
        writeln!(&mut wtr)?;
    }

    wtr.flush()
}
