use std::str::FromStr;

use std::io;

use aho_corasick::AhoCorasick;
use aho_corasick::AhoCorasickBuilder;
use aho_corasick::PatternID;

use std::collections::BTreeMap;

use crate::stat::patcnt::PatternStat;

pub enum MatchKind {
    Standard,
    LeftMostFirst,
    LeftMostLongest,
}

impl FromStr for MatchKind {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "std" => Ok(Self::Standard),
            "standard" => Ok(Self::Standard),

            "1st" => Ok(Self::LeftMostFirst),
            "first" => Ok(Self::LeftMostFirst),
            "mostfirst" => Ok(Self::LeftMostFirst),
            "leftmostfirst" => Ok(Self::LeftMostFirst),
            "most_first" => Ok(Self::LeftMostFirst),
            "left_most_first" => Ok(Self::LeftMostFirst),

            "long" => Ok(Self::LeftMostFirst),
            "longest" => Ok(Self::LeftMostFirst),
            "mostlongest" => Ok(Self::LeftMostFirst),
            "leftmostlongest" => Ok(Self::LeftMostFirst),
            "most_longest" => Ok(Self::LeftMostFirst),
            "left_most_longest" => Ok(Self::LeftMostFirst),

            _ => Err(io::Error::other(format!("invalid match kind: {s}"))),
        }
    }
}

impl From<MatchKind> for aho_corasick::MatchKind {
    fn from(m: MatchKind) -> Self {
        match m {
            MatchKind::Standard => Self::Standard,
            MatchKind::LeftMostFirst => Self::LeftmostFirst,
            MatchKind::LeftMostLongest => Self::LeftmostLongest,
        }
    }
}

pub fn args2patterns2stat_kind<L>(lines: L, kind: MatchKind) -> Result<PatternStat, io::Error>
where
    L: Iterator<Item = Result<Vec<u8>, io::Error>>,
{
    let patterns = std::env::args();
    let pairs = patterns.enumerate().map(|pair| {
        let (ix, keyword) = pair;
        (ix as u32, keyword)
    });
    let mut stat = PatternStat {
        id2pattern: BTreeMap::from_iter(pairs),
        stat: BTreeMap::new(),
    };

    let patterns = std::env::args();

    let ac: AhoCorasick = AhoCorasickBuilder::new()
        .match_kind(kind.into())
        .build(patterns)
        .map_err(io::Error::other)?;

    for rline in lines {
        let line: Vec<u8> = rline?;
        let found = ac.find_iter(&line);
        for item in found {
            let id: PatternID = item.pattern();
            let u: u32 = id.as_u32();

            let ocnt: Option<&mut u32> = stat.stat.get_mut(&u);
            match ocnt {
                None => {
                    stat.stat.insert(u, 1);
                }
                Some(cnt) => *cnt += 1,
            }
        }
    }

    Ok(stat)
}

pub fn args2patterns2stat<L>(lines: L) -> Result<PatternStat, io::Error>
where
    L: Iterator<Item = Result<Vec<u8>, io::Error>>,
{
    args2patterns2stat_kind(lines, MatchKind::Standard)
}
