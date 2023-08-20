use tauri::MenuItem;

use tauri::{AboutMetadata, CustomMenuItem, Menu, Submenu, WindowMenuEvent};

pub fn get_menu() -> Menu {
    let close = CustomMenuItem::new("close".to_string(), "关闭编辑器").accelerator("CmdOrCtrl+W");
    let copy_html_item =
        CustomMenuItem::new("copy_html".to_string(), "复制").accelerator("CmdOrCtrl+Shift+C");
    let new_file_item =
        CustomMenuItem::new("new_file".to_string(), "新建").accelerator("CmdOrCtrl+N");
    let save_file_item =
        CustomMenuItem::new("save_file".to_string(), "另存为...").accelerator("CmdOrCtrl+Shift+S");
    let save_html_file_item = CustomMenuItem::new("save_html_file".to_string(), "导出 HTML");
    let clear_storage = CustomMenuItem::new("clear_storage".to_string(), "清除缓存");
    let check_update = CustomMenuItem::new("check_update".to_string(), "检查更新");
    let first_menu = Menu::new()
        .add_native_item(MenuItem::About(
            "MDX Editor".to_string(),
            AboutMetadata::new(),
        ))
        .add_item(check_update)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::EnterFullScreen)
        .add_native_item(MenuItem::Minimize)
        .add_native_item(MenuItem::Hide)
        .add_native_item(MenuItem::HideOthers)
        .add_native_item(MenuItem::ShowAll)
        .add_native_item(MenuItem::Separator)
        .add_item(close)
        .add_native_item(MenuItem::Quit);

    let second_menu = Menu::new()
        .add_item(new_file_item)
        .add_item(copy_html_item)
        .add_item(save_file_item)
        .add_item(save_html_file_item)
        .add_item(clear_storage);

    let third_menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_native_item(MenuItem::Cut)
        .add_native_item(MenuItem::Paste)
        .add_native_item(MenuItem::Undo)
        .add_native_item(MenuItem::Redo)
        .add_native_item(MenuItem::SelectAll);

    let app_menu = Submenu::new("MDX Editor", first_menu);
    let file_menu = Submenu::new("文件", second_menu);
    let edit_menu = Submenu::new("编辑", third_menu);
    Menu::new()
        .add_submenu(app_menu)
        .add_submenu(file_menu)
        .add_submenu(edit_menu)
}

pub fn menu_event_handle(event: WindowMenuEvent) {
    if event.menu_item_id() == "close" {
        event.window().minimize().expect("can't minimize window");
        // event.window().eval("toggleVideoPlayback(true);").unwrap();
    }

    if event.menu_item_id() == "new_file" {
        let js_code = "newFile();";
        event.window().eval(js_code).unwrap();
    }

    if event.menu_item_id() == "copy_html" {
        let js_code = "handleCopy();";
        event.window().eval(js_code).unwrap();
    }

    if event.menu_item_id() == "clear_storage" {
        let js_code = "localStorage.clear(); window.location.reload();";
        event.window().eval(js_code).unwrap();
    }

    if event.menu_item_id() == "save_file" {
        let js_code = "handleExport();";
        event.window().eval(js_code).unwrap();
    }
    if event.menu_item_id() == "check_update" {
        let js_code = "checkUpdate();";
        event.window().eval(js_code).unwrap();
    }
    if event.menu_item_id() == "save_html_file" {
        let js_code = "handleExportHtml();";
        event.window().eval(js_code).unwrap();
    }
}
