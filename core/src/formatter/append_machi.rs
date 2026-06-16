/// 町名に「町」を補完します
/// 
/// This formatter appends "町" (machi) to the input string.
/// It's used to handle cases where town names ending with "町" 
/// have the suffix omitted in the address input.
///
/// Example:
/// - Input: "桜123" 
/// - Output: Some("桜町123")
pub(crate) fn append_machi(target: &str) -> Option<String> {
    Some(format!("{}町", target))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_machi_基本動作() {
        assert_eq!(append_machi("桜"), Some("桜町".to_string()));
        assert_eq!(append_machi("本"), Some("本町".to_string()));
        assert_eq!(append_machi("緑"), Some("緑町".to_string()));
    }

    #[test]
    fn append_machi_数字を含む場合() {
        assert_eq!(append_machi("桜123"), Some("桜町123".to_string()));
        assert_eq!(append_machi("本1-2-3"), Some("本町1-2-3".to_string()));
    }
}

