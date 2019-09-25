#[allow(dead_code)]
pub fn is_valid(code: String) -> bool {
    use regex::Regex;
    let cdata = Regex::new(r"<!\[CDATA\[.*?]]").unwrap();
    let test = cdata.replace_all(&code, "").to_string();
    let tag_regex = Regex::new(r"(<[A-Z]{1,9}>|</[A-Z]{2,10}>)").unwrap();
    let mut result = true;
    let mut stack = vec![];
    let mut should_close = false;
    for tag in tag_regex.captures_iter(&test) {
        let tag = &tag[0];
        if !should_close {
            if tag.contains('/') {
                result = false;
                break;
            }
            let tag_name = tag.replace(|c| c == '<' || c == '>', "");
            stack.push(tag_name);
            should_close = true;
        } else {
            if !tag.contains('/') {
                result = false;
                break;
            } else {
                let tag_name = tag.replace(|c| c == '<' || c == '>' || c == '/', "");
                if tag_name != stack.pop().unwrap() {
                    result = false;
                    break;
                } else {
                    should_close = false;
                }
            }
        }
    }
    result && {
        //这里假设传入的都是已经以tag包裹的字符串
        let test = tag_regex.replace_all(&test, "_");
        !test.contains('<')
    }
}

#[test]
fn test_is_valid() {
    let a = "<DIV>>>  ![cdata[]] <![CDATA[<div>]>]]>]]>>]</DIV>".to_owned();
    assert!(is_valid(a), "a is not a valid string");
    let a = "<A>  <B> </A>   </B>".to_owned();
    assert!(!is_valid(a), "a is  a valid string");
    let a = "<DIV>  div tag is not closed  <DIV>".to_owned();
    assert!(!is_valid(a), "a is  a valid string");
    let a = "<DIV>  unmatched <  </DIV>".to_owned();
    assert!(!is_valid(a), "a is  a valid string");
    let a = "<DIV> closed tags with invalid tag name  <b>123</b> </DIV>".to_owned();
    assert!(!is_valid(a), "a is  a valid string");
    let a = "<DIV> unmatched tags with invalid tag name  </1234567890> and <CDATA[[]]>  </DIV>"
        .to_owned();
    assert!(!is_valid(a), "a is  a valid string");
    let a = "<DIV>  unmatched start tag <B>  and unmatched end tag </C>  </DIV>".to_owned();
    assert!(!is_valid(a), "a is  a valid string");
}
