#![feature(allocator_api)]
use std::path::PathBuf;
use structopt::StructOpt;
mod app;
use app::TemplateApp;

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

    let mut app = TemplateApp::default();
    let theirs = diff::load(&opt.theirs).unwrap();
    let ours = diff::load(&opt.ours).unwrap();

    app.our_doc = theirs;
    app.their_doc = ours;
    eframe::run_native(Box::new(app));
}
