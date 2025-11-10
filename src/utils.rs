pub fn debug_json<T: serde::Serialize>(data: &T) {
    match serde_json::to_string_pretty(data) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Failed to serialize data to JSON: {}", e),
    }
}

pub fn debug_json_str(json_str: &str) {
    match serde_json::from_str::<serde_json::Value>(json_str) {
        Ok(value) => match serde_json::to_string_pretty(&value) {
            Ok(pretty) => println!("{}", pretty),
            Err(e) => eprintln!("Failed to format JSON: {}", e),
        },
        Err(e) => eprintln!("Failed to parse JSON string: {}", e),
    }
}
