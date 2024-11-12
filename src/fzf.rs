use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::cli::Editor;

pub fn open_fzf(index_path: &PathBuf, editor: &Editor) -> Result<(), Box<dyn Error>> {
    let index_path = index_path.to_string_lossy().to_string();

    let fzf_output = Command::new("fzf")
        .arg("--ansi")
        .arg("--delimiter=\\t")
        .arg("--no-scrollbar")
        .arg("--preview")
        .arg(format!(
            r#"{{
                grey="\033[90m"
                reset="\033[0m"
                filename=$(echo {{2}} | tr -d "''" | sed "s|$PWD\/||g")
                echo -n "${{grey}}${{filename}}${{reset}}\n\n" && \
                bat \
                    --color=always \
                    --highlight-line {{3}} \
                    --style=numbers \
                    --theme="Monokai Extended" \
                    {{2}}
            }}"#
        ))
        .arg("--preview-window=up,+{3},~2")
        .arg("--with-nth=1")
        .stdin(fs::File::open(index_path)?)
        .stdout(Stdio::piped())
        .spawn()?
        .wait_with_output()?;

    let selected = String::from_utf8_lossy(&fzf_output.stdout)
        .trim()
        .to_string();

    // Return if nothing was selected
    if selected.is_empty() {
        return Ok(());
    }

    // Extract path and line number
    let parts: Vec<&str> = selected.split('\t').collect();
    if parts.len() < 3 {
        eprintln!("[error] Problem parsing module path and line number");
        // TODO: Return error
        return Ok(());
    }

    let path = parts[1];
    let line = parts[2];

    // Exit if problems with path or line
    if path.is_empty() || line.is_empty() {
        eprintln!("[error] Problem parsing module path and line number");
        // TODO: Return error
        return Ok(());
    }

    let (command, arg) = match editor {
        Editor::Neovim => ("nvim", format!("+{} {}", line, path)),
        Editor::Vim => ("nvim", format!("+{} {}", line, path)),
        Editor::Vscode => ("open", format!("vscode://file/{}:{}", path, line)),
        Editor::Zed => ("zed", format!("{}:{}", path, line)),
    };

    Command::new(command).arg(arg).spawn()?.wait()?;

    Ok(())
}
