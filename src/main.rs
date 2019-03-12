//mod push_dominoes;
mod first_unique_character;
mod give_diamond;

fn main() {
//    let test = String::from("a.L.R...LR..L..");
    //    println!("{}", push_dominoes::push_dominoes(test));
    //    println!("{}",first_unique_character::first_uniq_char(test));
    let n:i32 = 5;
    if let Some(s) = give_diamond::print(5){
        println!("{}",s);
    }
}
