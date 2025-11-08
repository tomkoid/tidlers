pub fn save_session_data(ss: &str) {
    // save it to file
    std::fs::write("session.json", ss).unwrap();
}

pub fn remove_session_data() {
    if std::fs::remove_file("session.json").is_err() {
        println!("No session data to remove.");
    }
}

pub fn get_session_data() -> Option<String> {
    // read it from file if it exists
    std::fs::read_to_string("session.json").ok()
}
