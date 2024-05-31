use clap::Parser;
use hugr_cli::Level;

fn main() {
    let opts = hugr_mermaid::CmdLineArgs::parse();
    if let Err(e) = opts.run() {
        if opts.verbosity(Level::Error) {
            eprintln!("{}", e);
        }
        std::process::exit(1);
    }
}
