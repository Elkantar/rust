pub fn to_url(s: &str) -> String {
    // Use the `replace` method to replace white spaces with "%20"
    s.replace(" ", "%20")
}
