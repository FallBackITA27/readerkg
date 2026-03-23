use crate::args::Args;
mod args;
mod print_simple;
mod print_verbose;
mod render_video;
mod tabwriter;

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

    render_video::render(&ghost);

    print!(
        "{}",
        match final_args.verbose {
            false => print_simple::print_simple(ghost),
            true => print_verbose::print_verbose(ghost),
        }
    );
}
