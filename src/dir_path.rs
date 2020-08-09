use std::{
    fs,
    path::{Path, PathBuf},
};

// Struct for storing Directory path
pub(crate) struct DirPath {
    bin_dir: PathBuf,
    config_file: PathBuf,
    git_dir: PathBuf,
    checkout_dir: PathBuf,
    db_dir: PathBuf,
    registry_dir: PathBuf,
    cache_dir: PathBuf,
    index_dir: PathBuf,
    src_dir: PathBuf,
}

impl DirPath {
    // set directory path
    pub(crate) fn set_dir_path() -> Self {
        // set config file directory path
        let config_dir = dirs::config_dir().expect("Cannot get config directory location");
        // if config dir not exists create
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir).expect("Failed to create config dir");
        }
        let config_file = config_dir.join("cargo_trim_config.json");

        // change old config file to new config file location for mac os if it exists
        // TODO: remove this code block after >(^0.7.0) release
        if cfg!(target_os = "macos") {
            let mut old_config_file =
                dirs::preference_dir().expect("Cannot get preference directory location");
            old_config_file.push("cargo_trim_config.json");
            if old_config_file.exists() && !config_file.exists() {
                fs::rename(old_config_file, config_file.to_owned())
                    .expect("Cannot move old config file to new config file location");
            }
        }

        // If config file does not exists create config file
        if !config_file.exists() {
            fs::File::create(config_file.to_str().unwrap()).expect("Failed to create config file");
        }

        let home_dir = Path::new(env!("CARGO_HOME")).to_path_buf();

        // set bin directory path
        let bin_dir = home_dir.join("bin");

        // set git directory path
        let git_dir = home_dir.join("git");

        // set git dir sub folder path
        let checkout_dir = git_dir.join("checkouts");
        let db_dir = git_dir.join("db");

        // set registry dir path
        let registry_dir = home_dir.join("registry");

        // set registry dir sub folder path
        let cache_dir = registry_dir.join("cache");
        let src_dir = registry_dir.join("src");
        let index_dir = registry_dir.join("index");

        Self {
            bin_dir,
            config_file,
            git_dir,
            checkout_dir,
            db_dir,
            registry_dir,
            cache_dir,
            index_dir,
            src_dir,
        }
    }

    // return path of bin dir
    pub(crate) fn bin_dir(&self) -> &PathBuf {
        &self.bin_dir
    }

    // return path of config file
    pub(crate) fn config_file(&self) -> &PathBuf {
        &self.config_file
    }

    // return path of git dir
    pub(crate) fn git_dir(&self) -> &PathBuf {
        &self.git_dir
    }

    // return path of checkout dir
    pub(crate) fn checkout_dir(&self) -> &PathBuf {
        &self.checkout_dir
    }

    // return path of db dir
    pub(crate) fn db_dir(&self) -> &PathBuf {
        &self.db_dir
    }

    // return path of registry dir
    pub(crate) fn registry_dir(&self) -> &PathBuf {
        &self.registry_dir
    }

    // return path of cache dir
    pub(crate) fn cache_dir(&self) -> &PathBuf {
        &self.cache_dir
    }

    // return path of index dir
    pub(crate) fn index_dir(&self) -> &PathBuf {
        &self.index_dir
    }

    // return path of src dir
    pub(crate) fn src_dir(&self) -> &PathBuf {
        &self.src_dir
    }
}
