use rkg_utils::{footer::FooterType, header::in_game_time::InGameTime};

use crate::tabwriter::TabsWriter;
use std::fmt::Write;

pub fn print_simple(ghost: rkg_utils::Ghost) -> TabsWriter{
    let mut out = TabsWriter::default();

    let header = ghost.header();

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
