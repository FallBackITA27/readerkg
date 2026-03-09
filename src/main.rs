use std::fmt::Write;

use rkg_utils::{footer::{FooterType, sp_footer}, header::{combo::weight_class::GetWeightClass, location::constants::Version}};

fn convert_tabs_to_spaces(v: String) -> String {
    let lines = v.split('\n').collect::<Vec<&str>>();
    let line_count = lines.len();
    if line_count == 1 || line_count == 0 {
        return v.replace('\t', "  ");
    }

    let mut column_sizes = [0; 255];
    for line in &lines {
        let tabs = line.split('\t');
        for (idx, tab) in tabs.enumerate() {
            column_sizes[idx] = std::cmp::max(column_sizes[idx], tab.len() + 2);
        }
    }

    let mut out = String::new();

    for line in lines {
        let tabs = line.split('\t');
        for (idx, tab) in tabs.enumerate() {
            write!(out, "{}", tab).unwrap();
            for _ in 0..(column_sizes[idx] - tab.len()) {
                write!(out, " ").unwrap();
            }
        }
        writeln!(out).unwrap();
    }

    out
}

struct Args {
    file_path: Option<String>,
    verbose: bool,
}

impl Args {
    fn new_parse_args(args: &mut impl std::iter::Iterator<Item = String>) -> Self {
        let mut final_args = Self {
            file_path: None,
            verbose: false,
        };

        let mut args = args.peekable();
        while args.peek().is_some() {
            final_args.parse_args(&mut args);
        }

        final_args
    }

    fn parse_args(&mut self, args: &mut impl std::iter::Iterator<Item = String>) {
        match args.next().unwrap().as_str() {
            "-f" | "--file" | "--file-path" => self.file_path = Some(args.next().unwrap()),
            "-v" | "--verbose" => self.verbose = true,
            k => panic!("Unknown flag `{k}`!"),
        }
    }
}

fn main() {
    let mut args = std::env::args();
    let _exec = args.next();
    let final_args = Args::new_parse_args(&mut args);

    if final_args.file_path.is_none() {
        println!("No file path specified! Use `-f`, `--file` or `--file-path`");
        return;
    }

    let ghost = rkg_utils::Ghost::new_from_file(final_args.file_path.unwrap()).unwrap();
    println!("Ghost Read Successfully:");

    match final_args.verbose {
        false => print_simple(ghost),
        true => print_verbose(ghost),
    }
}

fn print_verbose(ghost: rkg_utils::Ghost) {
    let mut out = String::new();

    let header = ghost.header();

    writeln!(out, "Mii Name\t: {}", header.mii().name()).unwrap();

    let location = header.location();
    writeln!(out).unwrap();
    writeln!(out, "Country:").unwrap();
    fn next_ver(v: Version) -> Option<Version> {
        match v {
            Version::Vanilla => None,
            Version::ER10 => Some(Version::ER11),
            Version::ER11 => Some(Version::ER12),
            Version::ER12 => Some(Version::ER13),
            Version::ER13 => None,
        }
    }

    writeln!(
        out,
        "- {}\t: {}, {}",
        location.version(),
        location.subregion(),
        location.country(),
    )
    .unwrap();
    let mut version = location.version();
    while let Some(v) = next_ver(version) {
        let location = location.change_version(v);
        if location.is_none() {
            continue;
        }
        let location = location.unwrap();

        writeln!(
            out,
            "- {} : {},\t{}",
            location.version(),
            location.country(),
            location.subregion()
        )
        .unwrap();
        version = v;
    }

    writeln!(out, "Track Slot\t: {}", header.slot_id()).unwrap();
    writeln!(out, "Time\t: {}", header.finish_time()).unwrap();
    writeln!(out, "Lap Count\t: {}", header.lap_count()).unwrap();
    for (i, lap) in header.lap_split_times().iter().enumerate() {
        writeln!(out, "Lap {}\t: {}", i + 1, lap).unwrap();
    }

    let combo = header.combo();
    writeln!(out).unwrap();
    writeln!(out, "Combo:").unwrap();
    writeln!(out, "- Character\t: {}", combo.character()).unwrap();
    writeln!(out, "- Vehicle\t: {}", combo.vehicle()).unwrap();
    writeln!(out, "- Combo Weight Class\t: {}", combo.get_weight_class()).unwrap();
    writeln!(out, "- Vehicle Transmission\t: {}", combo.get_transmission()).unwrap();
    writeln!(out, "- Transmission Mod\t: {}", header.transmission_mod()).unwrap();
    writeln!(out, "- Transmission Adjusted to Mod\t: {}", header.transmission_adjusted()).unwrap();

    writeln!(out).unwrap();
    writeln!(out, "Controller\t: {}", header.controller()).unwrap();
    writeln!(out, "Ghost Type\t: {}", header.ghost_type()).unwrap();

    writeln!(out, "Date Set\t: {}", header.date_set()).unwrap();
    writeln!(
        out,
        "Drift Type\t: {}",
        if header.is_automatic_drift() {
            "Automatic"
        } else {
            "Manual"
        }
    )
    .unwrap();

    println!("{}", convert_tabs_to_spaces(out));
}

fn print_simple(ghost: rkg_utils::Ghost) {
    let header = ghost.header();
    let mut out = String::new();

    writeln!(out, "Mii Name\t: {}", header.mii().name()).unwrap();
    writeln!(out, "Track Slot\t: {}", header.slot_id()).unwrap();
    writeln!(out, "Time\t: {}", header.finish_time()).unwrap();
    for i in 0..header.lap_count() {
        writeln!(
            out,
            "Lap {}\t: {}",
            i + 1,
            header.lap_split_time(i as usize).unwrap()
        )
        .unwrap();
    }
    writeln!(out, "Date Set\t: {}", header.date_set()).unwrap();
    writeln!(out, "Combo\t: {}", header.combo()).unwrap();
    writeln!(out, "Controller\t: {}", header.controller()).unwrap();
    writeln!(
        out,
        "Drift Type\t: {}",
        if header.is_automatic_drift() {
            "Automatic"
        } else {
            "Manual"
        }
    )
    .unwrap();

    match ghost.footer() {
        None => (),
        Some(FooterType::CTGPFooter(ctgp_footer)) => {
            writeln!(out, "CTGP Footer Version\t: {}", ctgp_footer.footer_version()).unwrap();
        }
        Some(FooterType::SPFooter(sp_footer)) => {
            writeln!(out, "MKW-SP Footer Version\t: {}", sp_footer.footer_version()).unwrap();
        }
    };

    println!("{}", convert_tabs_to_spaces(out));
}
