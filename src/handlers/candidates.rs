pub fn handle_candidates() {
    eprintln!("Fetching candidates...");
    eprintln!("-------------------");
    let candidates = get_candidates();
    for candidate in candidates {
        eprintln!("{}", candidate);
    }
    eprintln!("-------------------");
}

pub fn get_candidates() -> Vec<std::string::String> {
    let url = "https://www.python.org/ftp/python/";
    let res = reqwest::blocking::get(url).unwrap().text().unwrap();
    let re = regex::Regex::new(r#"<a href="(\d+\.\d+\.\d+)/">"#).unwrap();
    let candidates: Vec<String> = re
        .captures_iter(&res)
        .map(|cap| cap[1].to_string())
        .collect();
    candidates
}
