use std::process::ExitCode;

use std::io;

use std::io::BufRead;

use std::io::BufWriter;
use std::io::Write;

use rs_count_keywords::keywords2stat::aho::corasick::words2stat;
use rs_count_keywords::stat::patcnt::PatternStat;
use words2stat::MatchKind;

use rs_count_keywords::stat::writer::stat2writer;

fn env_val_by_key(key: &'static str) -> Result<String, io::Error> {
    std::env::var(key).map_err(|e| io::Error::other(format!("env var {key} missing: {e}")))
}

fn match_kind() -> MatchKind {
    env_val_by_key("ENV_MATCH_KIND")
        .and_then(|s| str::parse(s.as_str()))
        .unwrap_or(MatchKind::Standard)
}

fn args2patterns2stdin2lines2stat() -> Result<PatternStat, io::Error> {
    let i = io::stdin();
    let il = i.lock();
    let lines = il.split(b'\n');
    words2stat::args2patterns2stat_kind(lines, match_kind())
}

fn stat2stdout(s: &PatternStat) -> Result<(), io::Error> {
    let o = io::stdout();
    let mut ol = o.lock();

    stat2writer(s, BufWriter::new(&mut ol))?;

    ol.flush()
}

fn sub() -> Result<(), io::Error> {
    let stat: PatternStat = args2patterns2stdin2lines2stat()?;
    stat2stdout(&stat)
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
