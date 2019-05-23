use regex::Regex;

pub fn mask_pii(s: String) -> String {
    if s.contains("@") {
        //email
        let email_reg = Regex::new(r"([a-zA-Z]{2,})@([a-zA-Z]{2,})\.([a-zA-Z]{2,})").unwrap();
        let caps = email_reg.captures(s.as_str()).unwrap();
        let user_name = caps.get(1).unwrap().as_str().chars().collect::<Vec<_>>();
        let mask_user_name =
            format!("{}*****{}", user_name[0], user_name.last().unwrap()).to_lowercase();
        format!(
            "{}@{}.{}",
            mask_user_name,
            caps.get(2).unwrap().as_str().to_lowercase(),
            caps.get(3).unwrap().as_str().to_lowercase()
        )
    } else {
        //phone
        let trim = Regex::new(r"[+\-,()\s]").unwrap();
        let trim_phone = trim.replace_all(s.as_str(), "");
        let length = trim_phone.len();
        let local = length - 10;
        let hide = length - 4;
        let prefix = if local > 0 {
            format!("+{}-", "*".repeat(local))
        } else {
            "".to_owned()
        };
        format!("{}***-***-{}", prefix, &trim_phone[hide..])
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_mask_pii() {
        let input = "LeetCode@LeetCode.com".to_owned();
        let output = "l*****e@leetcode.com".to_owned();
        assert_eq!(output, mask_pii(input));

        let input = "1(234)567-890".to_owned();
        let output = "***-***-7890".to_owned();
        assert_eq!(output, mask_pii(input));

        let input = "86-(10)12345678".to_owned();
        let output = "+**-***-***-5678".to_owned();
        assert_eq!(output, mask_pii(input));
    }
}
