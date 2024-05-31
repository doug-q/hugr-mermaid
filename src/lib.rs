use std::{error::Error, io::Write, process::Stdio};

use clap::{ArgMatches, Args, Command, FromArgMatches, Parser};
use hugr::HugrView;
use hugr_cli::Level;

#[derive(Debug)]
pub struct HugrCliCmdLineArgs(hugr_cli::CmdLineArgs);

#[derive(Debug,Parser)]
#[command(about,version)]
pub struct CmdLineArgs {
    #[clap(flatten)]
    pub base: HugrCliCmdLineArgs

}

impl CmdLineArgs {
    pub fn verbosity(&self, level: Level) -> bool {
        self.base.0.verbosity(level)
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>>{
        let hugr = self.base.0.run(&hugr::extension::EMPTY_REG)?;
        let outfile = tempfile::Builder::new().suffix(".png").tempfile()?;
        let mut cmd = std::process::Command::new("mmdc")
            .args(["-i","-","-o"]).arg(outfile.path())
            .stdin(Stdio::piped())
            .spawn()?;
        cmd.stdin.as_ref().unwrap().write_all(hugr.mermaid_string().as_bytes())?;
        if !cmd.wait()?.success() {
            Err("mmdc failed")?;
        }
        let mut config = viuer::Config::default();
        config.absolute_offset = false;
        viuer::print_from_file(outfile.path(), &config)?;
        Ok(())

    }
}

// impl HugrCliCmdLineArgs {
//     pub fn new() -> Self {
//         let x = hugr_cli::CmdLineArgs {
//             input: todo!(),
//             mermaid: todo!(),
//             no_validate: todo!(),
//             verbose: todo!(),
//         }
//     }
// }

impl FromArgMatches for HugrCliCmdLineArgs {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, clap::Error> {
        // let mut matches = matches.clone();
        // matches.remove_occurrences::<bool>("mermaid");
        Ok(HugrCliCmdLineArgs(hugr_cli::CmdLineArgs::from_arg_matches(matches)?))
    }

    fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> Result<(), clap::Error> {
        self.0.update_from_arg_matches(matches)
    }
}

impl Args for HugrCliCmdLineArgs {
    fn augment_args(cmd: Command) -> Command {
        hugr_cli::CmdLineArgs::augment_args(cmd).mut_arg("mermaid", |x| x.hide(true))
    }

    fn augment_args_for_update(cmd: Command) -> Command {
        hugr_cli::CmdLineArgs::augment_args_for_update(cmd).mut_arg("mermaid", |x| x.hide(true))
    }
}

