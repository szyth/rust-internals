fn audit_logs(entries: &Vec<String>) -> usize {
    entries.len()
}
fn main() {
    let logs = vec!["Log1".to_string(), "Log2".to_string(), "Log3".to_string()];

    // let _log_size = audit_logs(logs.clone()); // expensive; clone()
    let _log_size = audit_logs(&logs); // cheap; use reference
    println!("Logs: {:?}", logs);
}
