use indoc::indoc;

use crate::cli::Shell;

pub fn init(shell: &Shell) {
    let output = match shell {
        // TODO: add fish
        Shell::Bash => init_bash(),
        Shell::Zsh => init_zsh(),
    };
    println!("{}", output);
}

fn init_bash() -> String {
    indoc! { r#"
    # Add this to your ~/.bashrc
    # eval "$(exmodhop init bash)"

    _exmodhop_run() {
        exmodhop
    }
    bind -x '"\C-s": _exmodhop_run'
    "# }
    .to_string()
}

fn init_zsh() -> String {
    indoc! { r#"
    # Add this to your ~/.zshrc
    # eval "$(exmodhop init zsh)"

    _exmodhop_run() {
        exmodhop
        if [[ $? -ne 0 ]]; then echo "\n"; fi
        zle reset-prompt
    }
    zle -N _exmodhop_run
    bindkey '^S' _exmodhop_run
    "# }
    .to_string()
}
