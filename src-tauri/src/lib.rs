
use steamworks::Client;


#[tauri::command]
fn get_steam_id(client: tauri::State<Client>) -> String {
  client.user().steam_id().raw().to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let (client, _single) = Client::init_app(480).unwrap();

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_steam_id])
    .manage(client)
    
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
