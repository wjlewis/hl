pub fn escape_html(s: &str) -> String {
    s.replace("<", "&lt;").replace(">", "&gt;")
}