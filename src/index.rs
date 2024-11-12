use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Write};
use std::path::PathBuf;

use dashmap::DashMap;

pub type SourcePath = String;
pub type Module = (String, usize);

pub struct Index {
    index_path: PathBuf,
    bin_path: PathBuf,
    current_index: HashMap<SourcePath, Vec<Module>>,
    index_updates: DashMap<SourcePath, Vec<Module>>,
}

impl Index {
    pub fn new(root_path: &PathBuf) -> Self {
        let index_path = root_path.join("modules.index");
        let bin_path = root_path.join("modules.bin");

        let current_index = match Self::load_bin(&bin_path) {
            Ok(index) => index,
            Err(_) => HashMap::new(),
        };

        let index_updates = DashMap::new();

        Self {
            index_path,
            bin_path,
            current_index,
            index_updates,
        }
    }

    pub fn insert(&self, source_pathname: String, path_modules: Vec<Module>) {
        self.index_updates.insert(source_pathname, path_modules);
    }

    // TODO: return Result<(), _> {
    pub fn save(&mut self) {
        self.merge_updates();
        self.save_bin().expect("Failed to save bin");
        self.save_index().expect("Failed to save index");
    }

    fn load_bin(bin_path: &PathBuf) -> io::Result<HashMap<SourcePath, Vec<Module>>> {
        let file = File::open(bin_path)?;
        let reader = BufReader::new(file);
        bincode::deserialize_from(reader).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    fn merge_updates(&mut self) {
        for entry in self.index_updates.iter() {
            let (source_pathname, modules) = entry.pair();
            self.current_index
                .insert(source_pathname.clone(), modules.clone());
        }
    }

    fn save_bin(&self) -> io::Result<()> {
        let file = File::create(&self.bin_path)?;
        let mut writer = BufWriter::new(file);

        bincode::serialize_into(&mut writer, &self.current_index)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    fn save_index(&self) -> io::Result<()> {
        let mut entries: Vec<_> = self
            .current_index
            .iter()
            .flat_map(|(path, modules)| {
                modules
                    .iter()
                    .map(move |(module_name, line)| (module_name, path, line))
            })
            .collect();

        // Sort the entries by module_name
        entries.sort_by(|a, b| a.0.cmp(b.0));

        let file = File::create(&self.index_path)?;
        let mut writer = BufWriter::new(file);

        for (module_name, path, line) in entries {
            writeln!(writer, "{module_name}\t{path}\t{line}")?;
        }

        writer.flush()?;
        Ok(())
    }
}
