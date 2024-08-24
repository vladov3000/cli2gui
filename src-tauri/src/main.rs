use reqwest::blocking::Client;
use std::borrow::Borrow;
use std::process::Command;

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

struct State {
    client: Client,
}

#[tauri::command]
fn query(state: tauri::State<State>, content: &str) -> String {
    let client = &state.client;

    let body = format!(
        "{{
	 \"messages\": [
            {{
                \"role\": \"system\",
                \"content\": \"You are a frontend javascript developer. Output only production code. Do not write comments. Use let instead of const.\"
            }},
            {{
                \"role\": \"user\",
                \"content\": \"{}\"
            }}
        ],
        \"model\": \"meta-llama-3.1-70b-instruct\",
        \"max_tokens\": 1024,
        \"presence_penalty\": 0,
        \"temperature\": 0,
        \"top_p\": 1
	 }}",
        content
    );

    let request = client
        .post("https://text.octoai.run/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", AUTHORIZATION)
        .body(body);

    let response = request.send().unwrap().text().unwrap();
    response
}

#[tauri::command]
fn run(command: &str) -> String {
    let output = Command::new("/bin/zsh").arg("-c").arg(command).output().unwrap();
    let mut result = String::from_utf8_lossy(&output.stdout).to_string();
    result.push_str(String::from_utf8_lossy(&output.stderr).borrow());
    result
}

fn main() {
    let client = Client::new();

    let state = State { client };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![query, run])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

const AUTHORIZATION: &str = "Bearer eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6IjNkMjMzOTQ5In0.eyJzdWIiOiJhMmNhOTQ5ZS1jZjExLTQzZjAtYjUxYy00YTA2YzFjYjFlZGQiLCJ0eXBlIjoidXNlckFjY2Vzc1Rva2VuIiwidGVuYW50SWQiOiI3NzlhMjk1NS1jM2ZmLTQ4NGUtOThlMi02OThlODgzMTkwZmEiLCJ1c2VySWQiOiI4NTJmYTViZi0xZGRiLTQwZTctYWQxZS0wNWI4NTIwYmY3MDMiLCJhcHBsaWNhdGlvbklkIjoiYTkyNmZlYmQtMjFlYS00ODdiLTg1ZjUtMzQ5NDA5N2VjODMzIiwicm9sZXMiOlsiRkVUQ0gtUk9MRVMtQlktQVBJIl0sInBlcm1pc3Npb25zIjpbIkZFVENILVBFUk1JU1NJT05TLUJZLUFQSSJdLCJhdWQiOiIzZDIzMzk0OS1hMmZiLTRhYjAtYjdlYy00NmY2MjU1YzUxMGUiLCJpc3MiOiJodHRwczovL2lkZW50aXR5Lm9jdG8uYWkiLCJpYXQiOjE3MjQ1MjE0NzB9.E9R4hCB1M4wNYZu5CRhbuHyxY7SSjKNbPK3GPgc7fbsnKXLpPPqNmdpljDDXSPHz9GlxW-bIDPdoaRDkX3LgT7dJYM1xhtrvxpJMZf2WHWrrFLONh9tTPKrMx8OhC6Ht2Jx1GQq20BFR1glIznsPF9u7IsK2Aaj2oTLYctVRtKNBdwUpde4Ms1EK0Lep82BIN3b7jwXmqrJiHZB0Zs7nBP4c-SdIx8855T0OUk9nsfC1z6jo01OS0UFM-BrcTHPQLZIf2W3wU_Ab1tpY3CMTITeVT22ubaup1CjEw832XJP8cMXI-kS-2Y3yVcO6SPelEdK4_96NogTuYok4xaVJYw";
