use tauri::{CustomMenuItem, Menu, Submenu, WindowMenuEvent};

const EXIT: &str = "exit";

pub struct MainMenu {}

impl MainMenu {
  pub fn create_menu() -> Menu {
    let exit = CustomMenuItem::new(EXIT, "Exit");
    let submenu = Submenu::new("File", Menu::new().add_item(exit));
    Menu::new().add_submenu(submenu)
  }

  pub fn handler(event: WindowMenuEvent) {
    match event.menu_item_id() {
      EXIT => {
        event.window().close().unwrap();
      }
      _ => {}
    }
  }
}
