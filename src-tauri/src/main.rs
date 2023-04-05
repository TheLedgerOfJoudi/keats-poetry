use finder::find_poem;

mod finder;

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[tauri::command]
fn get_poem(title: &str) -> String {
    find_poem(title).expect("Should have been able to find a poem")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_poem])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
