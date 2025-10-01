use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClipboardEntry {
    pub id: String,
    pub content: String,
    pub content_type: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CliphistError {
    pub message: String,
}

impl From<std::io::Error> for CliphistError {
    fn from(err: std::io::Error) -> Self {
        CliphistError {
            message: format!("IO Error: {}", err),
        }
    }
}

fn run_cliphist_command(args: &[&str]) -> Result<String, CliphistError> {
    let output = Command::new("cliphist")
        .args(args)
        .output()
        .map_err(|e| CliphistError {
            message: format!("Failed to execute cliphist: {}. Make sure cliphist is installed.", e),
        })?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(CliphistError {
            message: format!("cliphist command failed: {}", String::from_utf8_lossy(&output.stderr)),
        })
    }
}

// Efficient fuzzy search combining multiple strategies
fn fuzzy_match(content: &str, query: &str) -> bool {
    if query.is_empty() {
        return true;
    }

    let content_lower = content.to_lowercase();
    let query_lower = query.to_lowercase();

    // Fast path 1: exact substring match
    if content_lower.contains(&query_lower) {
        return true;
    }

    // Fast path 2: word-based matching (split on whitespace/punctuation)
    let content_words: Vec<&str> = content_lower.split(|c: char| !c.is_alphanumeric()).collect();
    let query_words: Vec<&str> = query_lower.split(|c: char| !c.is_alphanumeric()).collect();

    // Check if all query words appear in content words (allows for typos within words)
    for query_word in &query_words {
        if query_word.is_empty() { continue; }
        let mut found = false;
        for content_word in &content_words {
            if content_word.contains(query_word) {
                found = true;
                break;
            }
        }
        if !found {
            return false;
        }
    }

    true
}

fn parse_cliphist_list(output: &str) -> Result<Vec<ClipboardEntry>, CliphistError> {
    let mut entries = Vec::new();

    for line in output.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 2 {
            let id = parts[0].to_string();
            // Join all remaining parts as content (in case content contains tabs)
            let content = parts[1..].join("\t");

            entries.push(ClipboardEntry {
                id,
                content: content.clone(), // Full content for display
                content_type: "text".to_string(), // Assume text for now
            });
        }
    }

    Ok(entries)
}

#[tauri::command]
fn is_cliphist_available() -> bool {
    run_cliphist_command(&["list"]).is_ok()
}

#[tauri::command]
fn get_history() -> Result<Vec<ClipboardEntry>, CliphistError> {
    let output = run_cliphist_command(&["list"])?;
    parse_cliphist_list(&output)
}



#[tauri::command]
fn get_entry_content(id: String) -> Result<String, CliphistError> {
    run_cliphist_command(&["decode", &id])
}

#[tauri::command]
fn delete_entry(id: String) -> Result<(), CliphistError> {
    use std::io::Write;
    use std::process::Stdio;

    let mut child = Command::new("cliphist")
        .arg("delete")
        .stdin(Stdio::piped())
        .spawn()
        .map_err(|e| CliphistError {
            message: format!("Failed to execute cliphist delete: {}", e),
        })?;

    if let Some(mut stdin) = child.stdin.take() {
        if let Err(e) = stdin.write_all(id.as_bytes()) {
            return Err(CliphistError {
                message: format!("Failed to write to cliphist stdin: {}", e),
            });
        }
    }

    let output = child.wait_with_output().map_err(|e| CliphistError {
        message: format!("Failed to wait for cliphist: {}", e),
    })?;

    if output.status.success() {
        Ok(())
    } else {
        Err(CliphistError {
            message: format!("cliphist delete failed: {}", String::from_utf8_lossy(&output.stderr)),
        })
    }
}

#[tauri::command]
fn search_history(query: String) -> Result<Vec<ClipboardEntry>, CliphistError> {
    let all_entries = get_history()?;
    let filtered = all_entries.into_iter()
        .filter(|entry| fuzzy_match(&entry.content, &query))
        .collect();
    Ok(filtered)
}

#[tauri::command]
fn copy_to_clipboard(content: String) -> Result<(), CliphistError> {
    use std::io::Write;

    // Try wl-copy first (Wayland)
    if let Ok(mut child) = Command::new("wl-copy")
        .stdin(std::process::Stdio::piped())
        .spawn()
    {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(content.as_bytes());
        }
        if child.wait().map(|s| s.success()).unwrap_or(false) {
            return Ok(());
        }
    }

    // Try xclip (X11)
    if let Ok(mut child) = Command::new("xclip")
        .args(&["-selection", "clipboard"])
        .stdin(std::process::Stdio::piped())
        .spawn()
    {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(content.as_bytes());
        }
        if child.wait().map(|s| s.success()).unwrap_or(false) {
            return Ok(());
        }
    }

    Err(CliphistError {
        message: "No clipboard tool available. Install wl-clipboard (Wayland) or xclip (X11).".to_string(),
    })
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            is_cliphist_available,
            get_history,
            get_entry_content,
            delete_entry,
            search_history,
            copy_to_clipboard
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
