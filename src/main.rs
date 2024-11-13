use std::path::PathBuf;
use std::{env, process};

use clap::Parser;
use exmodhop::cli::{Cli, Commands};
use exmodhop::fzf::open_fzf;
use exmodhop::init::init;
use exmodhop::project::Project;
use exmodhop::update_index;
use rayon::ThreadPoolBuilder;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init { shell }) => {
            init(shell);
            process::exit(0);
        }
        None => (),
    }

    let project_path = match &cli.project_path {
        Some(pathname) => PathBuf::from(pathname),
        None => env::current_dir().expect("Failed to get current directory"),
    };

    if !project_path.join("mix.exs").exists() {
        eprintln!("\n[error] Missing mix.exs in current project root");
        process::exit(1);
    }

    let project = Project::new(project_path);

    if cli.update_index {
        ThreadPoolBuilder::new()
            .num_threads(8)
            .build_global()
            .unwrap();

        update_index(&project); // .expect("Failed to update index");
    }

    if cli.open_fzf {
        open_fzf(&project.index_path, &cli.editor).expect("Failed to open fzf");
    }
}
