use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;
use tauri::{
    async_runtime::block_on,
    menu::{MenuBuilder, MenuEvent, SubmenuBuilder},
    App, AppHandle, Manager,
};
use tauri_plugin_dialog::DialogExt;

use crate::cache_config::CacheConfig;
use crate::game_folder::GameFolder;
use crate::save_as_pdx_script::save_as_pdx_script;

const OPEN_GAME_FOLDER: &str = "open-game-folder";
const OPEN_WORKING_DIRECTORY: &str = "open-working-directory";
const SAVE: &str = "save";
const EXIT: &str = "exit";

const DB_MIGRATE: &str = "db-migrate";
const DB_ROLLBACK: &str = "db-rollback";

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
        let database_menu = SubmenuBuilder::new(handle, "Database")
            .text(DB_MIGRATE, "Migrate")
            .text(DB_ROLLBACK, "Rollback")
            .build()?;

        let menu = MenuBuilder::new(handle)
            .item(&submenu)
            .item(&database_menu)
            .build()?;
        app.set_menu(menu).unwrap();
        Ok(())
    }

    pub fn handler(app_handle: &AppHandle, event: MenuEvent) {
        match event.id.as_ref() {
            OPEN_GAME_FOLDER => {
                handle_open_game_folder(app_handle);
            }
            OPEN_WORKING_DIRECTORY => handle_open_working_directory(app_handle),
            SAVE => {
                handle_save(app_handle);
            }
            EXIT => {
                app_handle.exit(1);
            }
            DB_MIGRATE => {
                handle_db_migrate(app_handle);
            }
            DB_ROLLBACK => {
                handle_db_rollback(app_handle);
            }
            _ => {}
        }
    }
}

fn handle_open_game_folder(app_handle: &AppHandle) {
    let app_handle = app_handle.clone();
    app_handle.dialog().file().pick_folder(move |file_path| {
        if let Some(file_path) = file_path {
            if let Ok(file_path) = file_path.into_path() {
                GameFolder {
                    folder_path: file_path,
                    app_handle,
                }
                .load()
            }
        }
    });
}

fn handle_open_working_directory(app_handle: &AppHandle) {
    let config_path = app_handle
        .path()
        .app_cache_dir()
        .unwrap()
        .join("config.json");

    app_handle.dialog().file().pick_folder(move |file_path| {
        if let Some(file_path) = file_path {
            if let Ok(file_path) = file_path.into_path() {
                let mut config: CacheConfig = match std::fs::read_to_string(&config_path) {
                    Ok(config) => serde_json::from_str(&config).unwrap(),
                    Err(_) => CacheConfig::new(),
                };

                config.working_dir = Some(file_path.clone());
                std::fs::write(config_path, serde_json::to_string(&config).unwrap()).unwrap();
            }
        }
    });
}

fn handle_save(app_handle: &AppHandle) {
    save_as_pdx_script(app_handle);
}

fn handle_db_migrate(app_handle: &AppHandle) {
    let db = app_handle.state::<DatabaseConnection>().inner();

    match block_on(Migrator::up(db, None)) {
        Ok(_) => {
            println!("Migration successful");
        }
        Err(e) => {
            println!("Migration failed: {}", e);
        }
    }
}

fn handle_db_rollback(app_handle: &AppHandle) {
    let db = app_handle.state::<DatabaseConnection>().inner();

    match block_on(Migrator::down(db, Some(1))) {
        Ok(_) => println!("Rollback successful"),
        Err(e) => println!("Rollback failed: {}", e),
    }
}
