use tauri::api::dialog::FileDialogBuilder;
use tauri::{CustomMenuItem, Manager, Menu, Submenu, WindowMenuEvent};

use crate::cache_config::CacheConfig;
use crate::game_folder::GameFolder;
use crate::save_as_pdx_script::save_as_pdx_script;

const OPEN_GAME_FOLDER: &str = "open-game-folder";
const OPEN_WORKING_DIRECTORY: &str = "open-working-directory";
const SAVE: &str = "save";
const EXIT: &str = "exit";

pub struct MainMenu {}

impl MainMenu {
    pub fn create_menu() -> Menu {
        let open_game_folder = CustomMenuItem::new(OPEN_GAME_FOLDER, "Open Game Folder");
        let open_working_directory =
            CustomMenuItem::new("open-working-directory", "Open Working Directory");
        let save = CustomMenuItem::new(SAVE, "Save");
        let exit = CustomMenuItem::new(EXIT, "Exit");
        let submenu = Submenu::new(
            "File",
            Menu::new()
                .add_item(open_game_folder)
                .add_item(open_working_directory)
                .add_item(save)
                .add_item(exit),
        );
        Menu::new().add_submenu(submenu)
    }

    pub fn handler(event: WindowMenuEvent) {
        match event.menu_item_id() {
            OPEN_GAME_FOLDER => {
                handle_open_game_folder(event);
            }
            OPEN_WORKING_DIRECTORY => handle_open_working_directory(event),
            SAVE => {
                handle_save(event);
            }
            EXIT => {
                event.window().close().unwrap();
            }
            _ => {}
        }
    }
}

fn handle_open_game_folder(event: WindowMenuEvent) {
    FileDialogBuilder::new().pick_folder(|file_path| {
        if let Some(file_path) = file_path {
            GameFolder {
                folder_path: file_path,
            }
            .load(event)
        }
    });
}

fn handle_open_working_directory(event: WindowMenuEvent) {
    FileDialogBuilder::new().pick_folder(move |file_path| {
        if let Some(file_path) = file_path {
            let config_path = event
                .window()
                .app_handle()
                .path_resolver()
                .app_cache_dir()
                .unwrap()
                .join("config.json");

            let mut config: CacheConfig = match std::fs::read_to_string(&config_path) {
                Ok(config) => serde_json::from_str(&config).unwrap(),
                Err(_) => CacheConfig::new(),
            };

            config.working_dir = Some(file_path.clone());
            std::fs::write(config_path, serde_json::to_string(&config).unwrap()).unwrap();
        }
    });
}

fn handle_save(event: WindowMenuEvent) {
    save_as_pdx_script(event);
}
