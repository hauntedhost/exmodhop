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
    # eval "$(fzf-ex-rs init bash)"

    run_fzf_ex_rs() {
        fzf-ex-rs
    }
    bind -x '"\C-s": run_fzf_ex_rs'
    "# }
    .to_string()
}

fn init_zsh() -> String {
    indoc! { r#"
    # Add this to your ~/.zshrc
    # eval "$(fzf-ex-rs init zsh)"

    run_fzf_ex_rs() {
        fzf-ex-rs
        if [[ $? -ne 0 ]]; then echo "\n"; fi
        zle reset-prompt
    }
    zle -N run_fzf_ex_rs
    bindkey '^S' run_fzf_ex_rs
    "# }
    .to_string()
}
