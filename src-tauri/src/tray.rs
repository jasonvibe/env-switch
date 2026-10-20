use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    AppHandle, Emitter, Manager,
};

pub fn create_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let show_item = MenuItem::with_id(app, "show", "打开控制面板", true, None::<&str>)?;
    let clear_item = MenuItem::with_id(app, "clear", "清除系统注入环境变量", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let quit_item = MenuItem::with_id(app, "quit", "退出 EnvSwitch", true, None::<&str>)?;

    let menu = Menu::with_items(
        app,
        &[&show_item, &clear_item, &separator, &quit_item],
    )?;

    let _tray = TrayIconBuilder::new()
        .tooltip("EnvSwitch 环境变量管理")
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "clear" => {
                let _ = app.emit("tray-clear-env", ());
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .build(app)?;

    Ok(())
}
