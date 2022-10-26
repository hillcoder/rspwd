use rustop::opts;
use rand::Rng;

fn main() {
    let (args, _rest) = opts! {
        synopsis "Commad line password generation.";
        opt upper:bool=false, desc:"no upper case";
        opt lower:bool=false, desc:"no lower case";
        opt number:bool=false, desc:"no number";
        opt symbol:bool=false, desc:"no symbol";
        opt password_len:usize=10,desc:"The number of lines.";
    }.parse_or_exit();

    let mut upper=String::from("");
    let mut lower=String::from("");
    let mut number=String::from("");
    let mut symbol=String::from("");

    if !args.upper { upper = String::from("QWERTYUIOPASDFGHJKLZXCVBNM");};
    if !args.lower { lower = String::from("qwertyuiopasdfghjklzxcvbnm");};
    if !args.number { number = String::from("1234567890");};
    if !args.symbol { symbol = String::from("!£$%&()=?^@#:;");};

    let mut total: String="".to_owned();
    total.push_str(&upper);
    total.push_str(&lower);
    total.push_str(&number);
    total.push_str(&symbol);
        
    let mut n = 0;
    let mut rng = rand::thread_rng();
    let mut result:String="".to_owned();
    while n < args.password_len {
        result.push(total.chars().nth(rng.gen_range(0..total.len()-1)).unwrap());
        n += 1;
    }

    println!("{}", result);
}
