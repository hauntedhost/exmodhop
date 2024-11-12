pub mod cli;
pub mod files;
pub mod fzf;
pub mod index;
pub mod init;
pub mod parser;
pub mod project;

use files::read_file_contents;
use index::Index;
use parser::{collect_path_modules, parse_elixir};
use project::Project;
use rayon::prelude::*;

// TODO: Return Result<(), _>
pub fn update_index(project: &Project) {
    let mut index = Index::new(&project.config_path);

    let paths = project
        .get_elixir_source_paths()
        .expect("Failed to get source code paths");

    paths.par_iter().for_each(|source_path| {
        let source_pathname = source_path.to_string_lossy().to_string();
        let source_code = read_file_contents(&source_path).expect("Failed to read source file");
        let tree = parse_elixir(&source_code);

        let mut path_modules = Vec::new();
        let mut module_parts = Vec::new();

        collect_path_modules(
            tree.root_node(),
            &source_code,
            &source_pathname,
            &mut module_parts,
            &mut path_modules,
        );

        index.insert(source_pathname, path_modules);
    });

    index.save(); // TODO: .expect("Failed to save index");
}
