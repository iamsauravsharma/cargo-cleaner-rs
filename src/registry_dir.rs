use crate::{list_crate, ConfigFile, CrateDetail};
use colored::*;
use std::{fs, path::Path};

// Stores .cargo/registry cache & src information
pub(crate) struct RegistryDir {
    cache_dir: String,
    src_dir: String,
}

impl RegistryDir {
    // Create new RegistryDir
    pub(crate) fn new(cache_dir: &Path, src_dir: &Path) -> Self {
        let cache_dir = cache_dir.to_str().unwrap().to_string();
        let src_dir = src_dir.to_str().unwrap().to_string();
        Self { cache_dir, src_dir }
    }

    // Remove crate from src & cache directory
    pub(crate) fn remove_crate(&self, crate_name: &str) {
        remove_crate(Path::new(&self.cache_dir), crate_name);
        remove_crate(Path::new(&self.src_dir), crate_name);
        println!("{} {:?}", "Removed".red(), crate_name);
    }

    // Get out src_dir path
    pub(crate) fn src(&self) -> &String {
        &self.src_dir
    }

    // Get out cache_dir path
    pub(crate) fn cache(&self) -> &String {
        &self.cache_dir
    }

    // Remove list of crates
    pub(crate) fn remove_crate_list(&self, crate_detail: &CrateDetail, list: &[String]) -> f64 {
        let mut size_cleaned = 0.0;
        for crate_name in list {
            self.remove_crate(crate_name);
            size_cleaned += crate_detail.find(crate_name, "REGISTRY")
        }
        size_cleaned
    }

    // Remove all crates from registry folder
    pub(crate) fn remove_all(
        &self,
        config_file: &ConfigFile,
        crate_name: &str,
        crate_detail: &CrateDetail,
    ) -> f64 {
        let crate_name = &crate_name.to_string();
        let mut sized_cleaned = 0.0;

        let read_include = config_file.include();
        let read_exclude = config_file.exclude();
        let crate_split = crate_name.rsplitn(2, '-');
        let mut simple_name = String::new();
        for (i, val) in crate_split.enumerate() {
            if i == 1 {
                simple_name = val.to_string()
            }
        }
        let env_include = list_crate::env_list("TRIM_INCLUDE");
        let env_exclude = list_crate::env_list("TRIM_EXCLUDE");

        if read_include.contains(crate_name)
            || read_include.contains(&simple_name)
            || env_include.contains(crate_name)
            || env_include.contains(&simple_name)
        {
            self.remove_crate(crate_name);
            sized_cleaned += crate_detail.find_size_registry_all(crate_name);
        }
        if !read_exclude.contains(crate_name)
            && !read_exclude.contains(&simple_name)
            && !env_exclude.contains(crate_name)
            && !env_exclude.contains(&simple_name)
        {
            self.remove_crate(crate_name);
            sized_cleaned += crate_detail.find_size_registry_all(crate_name);
        }
        sized_cleaned
    }
}

// // Use to open github folder present inside src and cache folder
// fn open_folder(path: &Path) -> Vec<String> {
//     // let mut path_buf = path.to_path_buf();
//     let mut folder_list = Vec::new();
//     for entry in fs::read_dir(path).unwrap() {
//         let entry = entry.unwrap();
//         let path = entry.path();
//         if path.is_dir() {
//             folder_list.push(path.to_str().unwrap().to_string());
//         }
//     }
//     folder_list
//     // path_buf.push("github.com-1ecc6299db9ec823");
//     // path_buf.to_str().unwrap().to_string()
// }

// Remove crates which name is provided to delete
fn remove_crate(path: &Path, value: &str) {
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.to_str().unwrap().contains(value) {
                if path.is_file() {
                    fs::remove_file(&path).unwrap();
                } else if path.is_dir() {
                    fs::remove_dir_all(&path).unwrap();
                }
            }
        }
    }
}
