#![feature(allocator_api)]
use app::DiffUiApp;
use env_logger;
use log::*;
use std::path::PathBuf;
use structopt::StructOpt;

mod app;
mod diff;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    /// theirs
    #[structopt(parse(from_os_str))]
    theirs: PathBuf,

    /// ours
    #[structopt(parse(from_os_str))]
    ours: PathBuf,

    /// result
    #[structopt(parse(from_os_str))]
    result: PathBuf,
}

fn main() {
    std::env::set_var("RUST_LOG", "info");
    let _ = env_logger::try_init();

    let opt = Opt::from_args();
    info!("{:?}", opt);

    // init aop with default
    let mut app = DiffUiApp::default();
    // set up the diff
    app.diff = diff::Diff::new(&opt.ours, &opt.theirs);
    app.our_doc = app.diff.ours.clone();
    app.their_doc = app.diff.theirs.clone();
    // egui default options
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(Box::new(app), native_options);
}
