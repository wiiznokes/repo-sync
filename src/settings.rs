extern crate directories;

use std::{
    fs::{self, File},
    path::PathBuf,
    process::exit,
};

use clap::{CommandFactory, Parser};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

static QUALIFIER: &str = "";
static ORG: &str = "";
static APP: &str = "repo-sync";

static SETTINGS_FILE: &str = "settings.toml";

pub fn get_global_config_path() -> PathBuf {
    ProjectDirs::from(QUALIFIER, ORG, APP)
        .unwrap()
        .config_dir()
        .to_path_buf()
}

pub fn get_global_settings_file() -> PathBuf {
    get_global_config_path().join(SETTINGS_FILE)
}

impl SettingsFile {
    /// create the file if needed, return the content or a default
    /// should be called once
    pub fn init_settings_file() -> SettingsFile {
        let config_path = get_global_config_path();
        if !config_path.exists() {
            if let Err(e) = fs::create_dir_all(config_path) {
                eprintln!("can't create settings directories: {e}");
                return SettingsFile::default();
            }
        }

        let settings_path = get_global_settings_file();

        if !settings_path.exists() {
            if let Err(e) = File::create(settings_path.clone()) {
                eprintln!("can't create settings file: {e}");
                return SettingsFile::default();
            }

            fs::write(
                settings_path,
                toml::to_string(&SettingsFile::default())
                    .expect("can't serialize settings default struct"),
            )
            .expect("can't write default settings content");

            SettingsFile::default()
        } else {
            Self::deserialize().unwrap_or_default()
        }
    }

    /// merge arg, exit if needeed
    pub fn merge_arg(&mut self, settings_arg: SettingsArg) {
        if let Some(repo_path) = settings_arg.repo_path {
            self.repo_path = repo_path;
        }
        if let Some(tpush) = settings_arg.tpush {
            self.tpush = tpush;
        }
        if let Some(tpull) = settings_arg.tpull {
            self.tpull = tpull;
        }

        if !self.repo_path.is_dir() {
            let mut command = SettingsArg::command();
            let help = command.render_help();
            println!("{}", help);
            exit(1)
        }
    }

    pub fn deserialize() -> Result<SettingsFile, ()> {
        match fs::read_to_string(get_global_settings_file()) {
            Ok(content) => match toml::from_str(&content) {
                Ok(settings) => Ok(settings),
                Err(e) => {
                    eprintln!("toml deserialization: {:?}", e);
                    Err(())
                }
            },
            Err(e) => {
                eprintln!("read settings file{:?}", e);
                Err(())
            }
        }
    }
}

/// this struct is merge to SettingsFile
#[derive(Parser, Debug)]
#[clap(author = "wiiznokes", version, about = "repo syncer", long_about = None)]
pub struct SettingsArg {
    #[arg(short = 'r', long = "repo", value_name = "PATH TO REPO")]
    pub repo_path: Option<PathBuf>,

    #[arg(long, value_name = "TIME BETWEEN PUSH(SECOND)")]
    pub tpush: Option<f32>,

    #[arg(long, value_name = "TIME BETWEEN PULL(SECOND)")]
    pub tpull: Option<f32>,
}

/// this one is use along the app.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SettingsFile {
    #[serde(default = "SettingsFile::empty")]
    pub repo_path: PathBuf,

    #[serde(default = "SettingsFile::one")]
    pub tpush: f32,

    #[serde(default = "SettingsFile::one")]
    pub tpull: f32,
}

impl Default for SettingsFile {
    fn default() -> Self {
        Self {
            repo_path: "".into(),
            tpush: 1f32,
            tpull: 1f32,
        }
    }
}

impl SettingsFile {
    fn empty() -> PathBuf {
        "".into()
    }

    fn one() -> f32 {
        1.0
    }
}
