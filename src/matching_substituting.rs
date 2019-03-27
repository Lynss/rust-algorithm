use regex::Regex;
use std::collections::HashMap;

fn my_change(s: &str, prog: &str, version: &str) -> String {
    let version = format!(" {}", version).to_owned();
    let prog = format!(" {}", prog).to_owned();
    let mut initial = s
        .split('\n')
        .map(|item| {
            if let [key, value] = *item.split(':').collect::<Vec<_>>().as_slice() {
                (key, value)
            } else {
                ("", "")
            }
        })
        .collect::<HashMap<&str, &str>>();

    let phone_regex = Regex::new(r"^\+1-\d{3}-\d{3}-\d{4}$").unwrap();
    let version_regex = Regex::new(r"^\d+\.\d+$").unwrap();
    if !phone_regex.is_match(initial["Phone"].trim())
        || !version_regex.is_match(initial["Version"].trim())
    {
        return "ERROR: VERSION or PHONE".into();
    };
    initial.entry("Version").and_modify(|n| {
        let nn = n.trim().parse::<f64>().unwrap();
        if nn != 2.0 {
            *n = version.as_str();
        };
    });
    initial.entry("Phone").and_modify(|p| {
        *p = " +1-503-555-0090";
    });
    initial.entry("Author").and_modify(|p| {
        *p = " g964";
    });
    initial.entry("Date").and_modify(|p| {
        *p = " 2019-01-01";
    });
    initial.entry("Program").or_insert(prog.as_str());
    initial.remove("Corporation");
    initial.remove("Level");
    format!(
        "Program:{} Author:{} Phone:{} Date:{} Version:{}",
        initial["Program"],
        initial["Author"],
        initial["Phone"],
        initial["Date"],
        initial["Version"]
    )
}

fn change(s: &str, prog: &str, version: &str) -> String {
    let re1 = Regex::new(r"Version: (?P<Version>\d+\.\d+)\n").unwrap();
    let cv = re1.captures(s);
    let re2 = Regex::new(r"Phone: (?P<Phone>\+1-\d{3}-\d{3}-\d{4})\n").unwrap();
    let cp = re2.captures(s);
    if cv.is_none() || cp.is_none() {
        return "ERROR: VERSION or PHONE".to_string();
    }
    let mut v = version; let p = "+1-503-555-0090";
    let text1 = cv.unwrap().name(&"Version").map_or("", |m| m.as_str());
    if text1 == "2.0" {
        v = "2.0";
    }
    return format!( "Program: {} Author: g964 Phone: {} Date: 2019-01-01 Version: {}", prog, p, v);
}

pub fn ms_print_change() {
    let s1 = "Program title: Primes\nAuthor: Kern\nCorporation: Gold\nPhone: +1-503-555-0091\nDate: Tues April 9, 2005\nVersion: 6.7\nLevel: Alpha";
    dbg!(change(s1, "Ladder", " 1.1"));
}
