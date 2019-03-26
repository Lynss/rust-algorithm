extern crate regex;

//mod push_dominoes;
//mod first_unique_character;
//mod give_diamond;
//mod playing_digits;
//mod diophantine_equation;
//mod your_order;
//mod consecutive_fib;
//mod move_square;
//mod algebraic_list;
//use algebraic_list::{Cons};
//mod recover_secret;

use regex::Regex;
use std::collections::HashMap;

fn main() {
    //    let test = String::from("a.L.R...LR..L..");
    //    println!("{}", push_dominoes::push_dominoes(test));
    //    println!("{}",first_unique_character::first_uniq_char(test));

    //    let n:i32 = 5;
    //    if let Some(s) = give_diamond::print(5){
    //        println!("{}",s);
    //    }

    //    let n = 89;
    //    let p = 1;
    //    println!("{}", playing_digits::dig_pow(n, p));

    //    let n: u64 = 90005;
    //    println!("{:?}", diophantine_equation::solequa(n));

    //    let sentence = "is2 Thi1s T4est 3a";
    //    println!("{}", your_order::order(sentence));

    //    let prod = 714;
    //    println!("{:?}", consecutive_fib::product_fib(prod));

    //    move_square::print_ms_result();

    //    let vec =vec![1, 2, 3, 4, 5];
    //    println!("{:?}", Cons::from_iter(vec.clone()).to_vec());
    //    println!("{:?}", Cons::from_iter(vec.clone()).filter(|&a|a>3).to_vec());
    //    println!("{:?}", Cons::from_iter(vec.clone()).map(|a|a+3).to_vec());

    //    recover_secret::rs_print();

    let s1 = "Program title: Primes\nAuthor: Kern\nCorporation: Gold\nPhone: +1-503-555-0091\nDate: Tues April 9, 2005\nVersion: 6.7\nLevel: Alpha";
    fn change(s: &str, prog: &str, version: &str) -> String {
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
        if !phone_regex.is_match(initial["Phone"].trim()) || !version_regex.is_match(initial["Version"].trim()) {
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
    dbg!(
        change(s1, "Ladder", " 1.1")
    );
}
