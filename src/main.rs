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
mod matching_substituting;

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
    matching_substituting::ms_print_change();
}
