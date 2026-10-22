use serde::{Serialize, Deserialize};
use std::path::Path;
use std::fs::File;
use std::io::ErrorKind;

#[derive(Debug, Clone)]
pub enum SaveError {
    Write,
    Format,
}

#[derive(Debug, Clone)]
pub enum LoadError {
    File,
    Format,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedSettings {
	pub theme: String,
}

impl SavedSettings {
    fn path(user_name: String) -> std::path::PathBuf {
        Path::new(&format!(r"C:\Users\x558899\Documents\code\rust\rust-learn\src\data\json\conf_{}.json", user_name)).to_path_buf()
    }

    pub async fn load(user_name: String) -> Result<SavedSettings, LoadError> {
        let contents = tokio::fs::read_to_string(Self::path(user_name.clone())).await;

        let ctn = match contents {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create(format!(r"C:\Users\x558899\Documents\code\rust\rust-learn\src\data\json\conf_{}.json", user_name)) {
                    Ok(fc) => {
                        let fallback = SavedSettings {
                            theme: String::from("Light")
                        };
                        let json = serde_json::to_string_pretty(&fallback).unwrap();
                        tokio::fs::write(Self::path(user_name.clone()), json.as_bytes()).await.unwrap();
                        return Ok(fallback);
                    },
                    Err(_) => return Err(LoadError::File),
                },
                _ => {
                    return Err(LoadError::File);
                }
            },
        };
        serde_json::from_str(&ctn).map_err(|_| LoadError::Format)
    }

    pub async fn save(self, user_name: String) -> Result<(), SaveError> {
        use iced::time::milliseconds;

        let json = serde_json::to_string_pretty(&self)
            .map_err(|_| SaveError::Format)?;

        let path = Self::path(user_name);

        if let Some(dir) = path.parent() {
            tokio::fs::create_dir_all(dir)
                .await
                .map_err(|_| SaveError::Write)?;
        }

        {
            tokio::fs::write(path, json.as_bytes())
                .await
                .map_err(|_| SaveError::Write)?;
        }

        // This is a simple way to save at most twice every second
        tokio::time::sleep(milliseconds(500)).await;

        Ok(())
    }
}
