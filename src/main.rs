use std::path::PathBuf;
use structopt::StructOpt;
mod app;
use app::TemplateApp;
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

    let app = TemplateApp::default();
    eframe::run_native(Box::new(app));
}