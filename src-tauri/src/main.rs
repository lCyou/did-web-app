// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod print;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_keypair,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn create_keypair() {
  use did_key::*;
  let key = generate::<Secp256k1KeyPair>(None);

  println!("{}", key.fingerprint());

  let did_doc = key.get_did_document(Config {
    use_jose_format: true,    // toggle to switch between LD and JOSE key format
    serialize_secrets: false  // toggle to serialize private keys
  });
  let doc_json = serde_json::to_string_pretty(&did_doc).unwrap();

  println!("{}", doc_json);

}
