//正则表达式判断
pub fn validate(pattern: &str, content: &str) -> bool {
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    use regex::Regex;
    #[cfg(any(target_os = "linux"))]
    use tauri::regex::Regex;
    let re = Regex::new(pattern).unwrap();
    re.is_match(content)
}
#[test]
fn test() {
    println!("test-- {}", validate(r#"(?:md)$"#, "44.jpg.33.JPEG.md"));
}
