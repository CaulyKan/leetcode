impl Solution {
    pub fn entity_parser(text: String) -> String {
        let s1 = text.replace("&quot;", "\"");
        let s1 = s1.replace("&apos;", "'");
        let s1 = s1.replace("&amp;", "&");
        let s1 = s1.replace("&gt;", ">");
        let s1 = s1.replace("&lt;", "<");
        let s1 = s1.replace("&frasl;", "/");
        s1
    }
}

pub struct Solution;
