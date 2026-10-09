pub mod commands;
pub mod core;
mod settings;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(core::runner::LauncherState(std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new()))))
        .invoke_handler(tauri::generate_handler![
            settings::get_settings,
            settings::save_settings,
            core::instances::get_instances_cmd,
            core::instances::create_instance,
            core::instances::rename_instance,
            core::instances::delete_instance,
            core::instances::open_instance_folder,
            core::runner::launch_instance,
            core::runner::kill_instance,
            core::runner::get_running_instances,
            core::forge::fetch_forge_versions,
            commands::accounts::get_accounts,
            commands::accounts::add_account,
            commands::accounts::add_microsoft_account,
            commands::accounts::select_account,
            commands::accounts::delete_account,
            commands::auth::start_microsoft_login,
            commands::auth::cancel_microsoft_login,
            commands::auth::refresh_account_token,
            commands::servers::get_instance_servers,
            commands::servers::add_instance_server,
            commands::servers::update_instance_server,
            commands::servers::remove_instance_server,
            commands::servers::update_server_icon,
            commands::servers::ping_server,
            commands::worlds::get_instance_worlds,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
