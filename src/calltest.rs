//! `--calltest <number> <id> [D1] [write]`: dial a phone through the modem in TFW.INI and run
//! the reads the Direct windows use (and with `write`, write back what was read), printing
//! each result. For testing against a phone or the Ernest D3 emulator.

use crate::fea::FeatureFile;
use crate::modem::{DialConf, Job, Modem, Op, Res, Target};
use crate::proto::Map;
use crate::text::Profile;
use std::time::{Duration, Instant};

fn wait(job: &Job, what: &str) -> Option<Res> {
    let t0 = Instant::now();
    let mut seen = 0;
    loop {
        {
            let s = job.state.lock().unwrap();
            for l in &s.lines[seen..] {
                println!("      {l}");
            }
            seen = s.lines.len();
        }
        if let Some(r) = job.take() {
            let secs = t0.elapsed().as_secs_f32();
            return match r {
                Ok(r) => {
                    println!("OK   {what} ({secs:.1} s): {}", show(&r));
                    Some(r)
                }
                Err(e) => {
                    println!("FAIL {what} ({secs:.1} s): {e}");
                    None
                }
            };
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}

fn show(r: &Res) -> String {
    match r {
        Res::Areas(a) => format!("{} areas, {} bytes", a.len(), a.iter().map(|x| x.1.len()).sum::<usize>()),
        Res::Rate(b) => format!("{} bytes", b.len()),
        Res::Smdr(v) => format!("{} calls {:?}", v.len(), v.iter().take(3).collect::<Vec<_>>()),
        Res::Csn(v) => format!("{} entries {:?}", v.len(), v.iter().take(3).collect::<Vec<_>>()),
        Res::Collection(c) => format!("box {:?} cumulative {:?} refunds {:?}/{:?} raw {:02X?}", c.coin_box_cents, c.cumulative_cents, c.refund_calls, c.refund_cents, c.raw),
        other => format!("{other:?}"),
    }
}

pub fn run(args: &[String]) -> i32 {
    let number = args.first().cloned().unwrap_or_default();
    let id = args.get(1).cloned().unwrap_or_default();
    let d3 = !args.iter().any(|a| a.eq_ignore_ascii_case("D1"));
    let write = args.iter().any(|a| a == "write");
    let p = Profile::new();
    let line = crate::modem::LineChoice::from_profile(&p);
    let label = line.label();
    let m = Modem::start();
    m.open(line);
    std::thread::sleep(Duration::from_millis(500));
    if let Some(e) = m.sh.lock().unwrap().open_error.clone() {
        println!("FAIL open {label}: {e}");
        return 1;
    }
    println!("{label} open; dialing {number}, ID {id}, {}", if d3 { "D3" } else { "D1" });
    let conf = DialConf {
        init_addons: p.get("Modem", "ModemInitAddons"),
        d3_addons: p.get("Modem", "ModemD3InitAddons"),
        d1_addons: p.get("Modem", "ModemD1InitAddons"),
        pbx: p.get("DialingDefaults", "PBX"),
        dcw: p.get("DialingDefaults", "DCW"),
        timeout: p.get("Dialtime", "OnePlus").trim().parse::<u32>().unwrap_or(121) + 5,
    };
    let t = Target { dial: number.clone(), shown: number.clone(), id, d3 };
    let Some(Res::Connected { firmware }) = wait(&m.dial(t, conf), "connect") else { return 1 };
    let mut fails = 0;
    let mut run = |op: Op, what: &str| -> Option<Res> {
        let r = wait(&m.run(op), what);
        if r.is_none() {
            fails += 1;
        }
        r
    };
    run(Op::ReadFirmware, "firmware versions");
    let sd = run(Op::ReadSpeedDial, "speed dial");
    let clock = run(Op::ReadClock, "time and date");
    let scroll = run(Op::ReadScroll, "scrolling message");
    run(Op::ReadCollection, "collection");
    run(Op::ReadSmdr, "SMDR");
    let csn = run(Op::ReadCsn, "CSN table");
    // Programmable features: the .FEA file for this firmware, its pages (frmProgrammableFeatures).
    let dir = crate::text::path("FEA");
    let fea = crate::forms::lists::find_fea_for(&dir, &firmware, d3).map(|(p, f)| {
        println!("      features file {}", p.display());
        f
    });
    let mut fea_read: Option<(FeatureFile, Vec<(u16, Vec<u8>)>)> = None;
    match fea {
        Some(f) => {
            let addrs: Vec<(u16, usize)> = f.features.iter().map(|x| (x.address(), x.len())).collect();
            if let Some(Res::Areas(a)) = run(Op::ReadAreas(Map { d3 }.feature_pages(&addrs)), "programmable features") {
                fea_read = Some((f, a));
            }
        }
        None => println!("SKIP programmable features: no .FEA file for {firmware} in {}", dir.display()),
    }
    if write {
        if let Some((f, areas)) = &fea_read {
            // Download what was read, every option but the version, as the Download menu does.
            let recs: Vec<(u16, Vec<u8>)> = f
                .features
                .iter()
                .filter(|x| !x.read_only())
                .filter_map(|x| {
                    let (a, n) = (x.address() as usize, x.len());
                    areas.iter().find(|(s, d)| a >= *s as usize && a + n <= *s as usize + d.len()).map(|(s, d)| (x.address(), d[a - *s as usize..a - *s as usize + n].to_vec()))
                })
                .collect();
            println!("      {} options to write", recs.len());
            run(Op::WriteRecords(recs), "programmable features download");
        }
        if let Some(Res::SpeedDial(n)) = sd {
            run(Op::WriteSpeedDial(n), "write speed dial back");
        }
        if clock.is_some() {
            // The current time, as Download Time & Date with "Current Date & Time" sends it.
            use chrono::{Datelike, Timelike};
            let n = chrono::Local::now().naive_local();
            let c = crate::proto::Clock {
                yy: (n.year() % 100) as u8,
                mm: n.month() as u8,
                dd: n.day() as u8,
                day: crate::proto::day_code(n.weekday().num_days_from_sunday()),
                hh: n.hour() as u8,
                mi: n.minute() as u8,
                ss: n.second() as u8,
            };
            run(Op::WriteClock(c), "write time and date");
            run(Op::ReadClock, "time and date after setting");
        }
        if let Some(Res::Scroll(s)) = scroll {
            run(Op::WriteScroll(s), "write scrolling message back");
        }
        if let Some(Res::Csn(rows)) = csn {
            let e: Result<Vec<Vec<u8>>, String> = rows.iter().map(|r| crate::proto::csn_bytes(&r.0, &r.1, &r.2, &r.3, &r.4, &r.5)).collect();
            match e {
                Ok(e) => {
                    run(Op::WriteCsn(e), "write CSN table back");
                }
                Err(e) => println!("SKIP write CSN: {e}"),
            }
        }
    }
    m.hang_up();
    std::thread::sleep(Duration::from_secs(4));
    println!("done, {fails} failed");
    fails
}
