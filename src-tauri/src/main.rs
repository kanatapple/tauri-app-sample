#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::http::ResponseBuilder;

fn main() {
    tauri::Builder::default()
        .register_uri_scheme_protocol("example", move |_app_handle, request| {
            println!("Receive custom scheme: {:?}", request);
            ResponseBuilder::new().status(200).body(Vec::new())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
