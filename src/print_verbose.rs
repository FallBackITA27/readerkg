use std::fmt::Write;

use rkg_utils::{footer::FooterType, header::{in_game_time::InGameTime, location::constants::Version}};

use crate::tabwriter::TabsWriter;

pub fn print_verbose(ghost: rkg_utils::Ghost) -> TabsWriter {
    let mut out = TabsWriter::default();

    let header = ghost.header();
    let footer = ghost.footer();

    writeln!(out, "Mii Name\t: {}", header.mii().name()).unwrap();

    let location = header.location();
    writeln!(out).unwrap();
    writeln!(out, "- Country:").unwrap();
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
        "{}\t: {}, {}",
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

    writeln!(out).unwrap();
    writeln!(out, "- Time Data:").unwrap();
    writeln!(out, "Track Slot\t: {}", header.slot_id()).unwrap();

    match footer {
        None => writeln!(out, "Time\t: {}", header.finish_time()).unwrap(),
        Some(FooterType::SPFooter(sp_footer)) => writeln!(out, "Time\t: {}", sp_footer.exact_finish_time()).unwrap(),
        Some(FooterType::CTGPFooter(ctgp_footer)) => writeln!(out, "Time\t: {}", ctgp_footer.exact_finish_time()).unwrap(),
    }
    writeln!(out, "Lap Count\t: {}", header.lap_count()).unwrap();
    match footer {
        None => {
            for (i, lap) in header.lap_split_times().iter().enumerate() {
                writeln!(out, "Lap {}\t: {}", i + 1, lap).unwrap();
            }
        }
        Some(FooterType::SPFooter(sp_footer)) => {
            for (i, lap) in sp_footer.exact_lap_times().iter().enumerate() {
                writeln!(out, "Lap {}\t: {}", i + 1, lap).unwrap();
            }
        }
        Some(FooterType::CTGPFooter(ctgp_footer)) => {
            for (i, lap) in ctgp_footer.exact_lap_times().iter().enumerate() {
                writeln!(out, "Lap {}\t: {}", i + 1, lap).unwrap();
            }
        }
    }

    let combo = header.combo();
    writeln!(out).unwrap();
    writeln!(out, "- Combo:").unwrap();
    writeln!(out, "Character\t: {}", combo.character()).unwrap();
    writeln!(out, "Vehicle\t: {}", combo.vehicle()).unwrap();
    writeln!(out, "Combo Weight Class\t: {}", combo.get_weight_class()).unwrap();
    writeln!(out, "Vehicle Transmission\t: {}", combo.get_transmission()).unwrap();
    writeln!(out, "Transmission Mod\t: {}", header.transmission_mod()).unwrap();
    writeln!(
        out,
        "Transmission Adjusted to Mod\t: {}",
        header.transmission_adjusted()
    )
    .unwrap();

    writeln!(out).unwrap();
    writeln!(out, "- Miscellaneous").unwrap();
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
    writeln!(out, "Compressed Data\t: {}", header.is_compressed()).unwrap();

    writeln!(out).unwrap();
    writeln!(out, "- Extra Data:").unwrap();
    match ghost.footer() {
        None => (),
        Some(FooterType::CTGPFooter(ctgp_footer)) => {
            writeln!(
                out,
                "CTGP Footer Version\t: {}",
                ctgp_footer.footer_version()
            )
            .unwrap();
            writeln!(
                out,
                "Time Paused\t: {}",
                InGameTime::from_milliseconds(
                    ctgp_footer.rtc_time_paused().num_milliseconds() as u32
                )
            )
            .unwrap();
            writeln!(out, "Shroomstrat\t: {}", ctgp_footer.shroomstrat_string()).unwrap();
            writeln!(
                out,
                "Potentially Cheated\t: {}",
                ctgp_footer.potentially_cheated_ghost()
            )
            .unwrap();
            writeln!(out, "Category\t: {}", (ctgp_footer.category())).unwrap();
        }
        Some(FooterType::SPFooter(sp_footer)) => {
            writeln!(
                out,
                "MKW-SP Footer Version\t: {}",
                sp_footer.footer_version()
            )
            .unwrap();
            if let Some(v) = sp_footer.is_vanilla_mode_enabled() {
                writeln!(out, "Vanilla Mode\t: {v}").unwrap();
            }
            if let Some(v) = sp_footer.shroomstrat_string() {
                writeln!(out, "Shroomstrat\t: {v}").unwrap();
            }
            writeln!(out, "Speed Mod\t: {}", sp_footer.has_speed_mod()).unwrap();
            writeln!(out, "Ultra\t: {}", sp_footer.has_ultra_shortcut()).unwrap();
            writeln!(out, "Wallride\t: {}", sp_footer.has_wallride()).unwrap();
            writeln!(
                out,
                "Horizontal Wall Glitch\t: {}",
                sp_footer.has_horizontal_wall_glitch()
            )
            .unwrap();
        }
    };

    out
}

