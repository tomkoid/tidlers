/// Prints JSON data in a formatted way for debugging
pub(crate) fn _debug_json<T: serde::Serialize>(data: &T) {
    match serde_json::to_string_pretty(data) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Failed to serialize data to JSON: {}", e),
    }
}

/// Parses and prints a JSON string in a formatted way, also saves to a debug file
pub(crate) fn debug_json_str(json_str: &str) {
    match serde_json::from_str::<serde_json::Value>(json_str) {
        Ok(value) => match serde_json::to_string_pretty(&value) {
            Ok(pretty) => println!("{}", pretty),
            Err(e) => eprintln!("Failed to format JSON: {}", e),
        },
        Err(e) => eprintln!("Failed to parse JSON string: {}", e),
    }

    // save the json to a debug file
    save_json_to_file(json_str);
}

const DEFAULT_DEBUG_FILE: &str = "debug_output.json";

fn save_json_to_file(data: &str) {
    if let Err(e) = std::fs::write(DEFAULT_DEBUG_FILE, data) {
        eprintln!("Failed to write JSON to file {}: {}", DEFAULT_DEBUG_FILE, e);
    }
}
