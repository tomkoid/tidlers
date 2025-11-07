pub fn save_session_data(ss: &str) {
    // save it to file
    std::fs::write("session.json", ss).unwrap();
}

pub fn get_session_data() -> Option<String> {
    // read it from file if it exists
    std::fs::read_to_string("session.json").ok()
}
