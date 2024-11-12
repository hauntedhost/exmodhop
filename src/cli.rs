use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[arg(long, help = "Path to Elixir project [default: current directory]")]
    pub project_path: Option<String>,

    #[arg(long, default_value_t = true, help = "Update index [default: true]")]
    pub update_index: bool,

    #[arg(
        long,
        default_value_t = true,
        help = "Open fzf with module index [default: true]"
    )]
    pub open_fzf: bool,

    #[arg(
        long,
        value_enum,
        default_value_t = Editor::Vscode,
        help = "Editor to open source files"
    )]
    pub editor: Editor,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "Show shell init script")]
    Init {
        #[arg(value_enum, default_value_t = Shell::Zsh, help = "Shell (Default zsh)")]
        shell: Shell,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Editor {
    Neovim,
    Vim,
    Vscode,
    Zed,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Shell {
    Bash,
    Zsh,
}
