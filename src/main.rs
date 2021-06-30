#![feature(allocator_api)]
use std::path::PathBuf;
use structopt::StructOpt;
mod app;
use app::DiffUiApp;

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
    let opt = Opt::from_args();
    println!("{:?}", opt);

    // init aop with default
    let mut app = DiffUiApp::default();
    let theirs = diff::load(&opt.theirs).unwrap();
    let ours = diff::load(&opt.ours).unwrap();

    // init diff
    let mut d = diff::Diff::default();
    d.add_doc("theirs", theirs);
    d.add_doc("ours", ours);
    d.read();


    app.diff = d;
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(Box::new(app), native_options);
}
