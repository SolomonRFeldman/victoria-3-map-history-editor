use tauri::api::dialog::FileDialogBuilder;
use tauri::{
    menu::{MenuBuilder, MenuEvent, SubmenuBuilder},
    App, AppHandle, Manager,
};

use crate::cache_config::CacheConfig;
use crate::game_folder::GameFolder;
use crate::save_as_pdx_script::save_as_pdx_script;

const OPEN_GAME_FOLDER: &str = "open-game-folder";
const OPEN_WORKING_DIRECTORY: &str = "open-working-directory";
const SAVE: &str = "save";
const EXIT: &str = "exit";

pub struct MainMenu {}

impl MainMenu {
    pub fn create_menu(app: &App) -> Result<(), tauri::Error> {
        let handle = app.handle();
        let submenu = SubmenuBuilder::new(handle, "File")
            .text(OPEN_GAME_FOLDER, "Open Game Folder")
            .text("open-working-directory", "Open Working Directory")
            .text(SAVE, "Save")
            .text(EXIT, "Exit")
            .build()?;
        let menu = MenuBuilder::new(handle).item(&submenu).build()?;
        app.set_menu(menu);
        Ok(())
    }

    pub fn handler(app: &AppHandle, event: MenuEvent) {
        match event.id.as_ref() {
            OPEN_GAME_FOLDER => {
                handle_open_game_folder(event);
            }
            OPEN_WORKING_DIRECTORY => handle_open_working_directory(app),
            SAVE => {
                handle_save(event);
            }
            EXIT => {
                app.exit(1);
            }
            _ => {}
        }
    }
}

fn handle_open_game_folder(event: MenuEvent) {
    FileDialogBuilder::new().pick_folder(|file_path| {
        if let Some(file_path) = file_path {
            GameFolder {
                folder_path: file_path,
            }
            .load(event)
        }
    });
}

fn handle_open_working_directory(app: &AppHandle) {
    FileDialogBuilder::new().pick_folder(move |file_path| {
        if let Some(file_path) = file_path {
            let config_path = app.path().app_cache_dir().unwrap().join("config.json");

            let mut config: CacheConfig = match std::fs::read_to_string(&config_path) {
                Ok(config) => serde_json::from_str(&config).unwrap(),
                Err(_) => CacheConfig::new(),
            };

            config.working_dir = Some(file_path.clone());
            std::fs::write(config_path, serde_json::to_string(&config).unwrap()).unwrap();
        }
    });
}

fn handle_save(event: MenuEvent) {
    save_as_pdx_script(event);
}
