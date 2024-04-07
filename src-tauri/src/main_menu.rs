use tauri::{CustomMenuItem, Menu, Submenu, WindowMenuEvent};
use tauri::api::dialog::FileDialogBuilder;

use crate::game_folder::GameFolder;

const OPEN_GAME_FOLDER: &str = "open-game-folder";
const EXIT: &str = "exit";

pub struct MainMenu {}

impl MainMenu {
  pub fn create_menu() -> Menu {
    let open_game_folder = CustomMenuItem::new(OPEN_GAME_FOLDER, "Open Game Folder");
    let exit = CustomMenuItem::new(EXIT, "Exit");
    let submenu = Submenu::new("File", Menu::new().add_item(open_game_folder).add_item(exit));
    Menu::new().add_submenu(submenu)
  }

  pub fn handler(event: WindowMenuEvent) {
    match event.menu_item_id() {
      OPEN_GAME_FOLDER => { handle_open_game_folder(event); },
      EXIT => { event.window().close().unwrap(); }
      _ => {}
    }
  }
}

fn handle_open_game_folder(event: WindowMenuEvent) {
  FileDialogBuilder::new().pick_folder(|file_path| {
    if let Some(file_path) = file_path {
      GameFolder { folder_path: file_path }.load(event)
    }
  });
}
