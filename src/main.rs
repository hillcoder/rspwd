use rustop::opts;
use rand::Rng;

fn main() {
    let (args, _rest) = opts! {
        synopsis "Commad line password generation.";
        opt upper:bool=false, desc:"use upper case";
        opt lower:bool=false, desc:"use lower case";
        opt number:bool=false, desc:"use number";
        opt symbol:bool=false, desc:"use symbol";
        opt password_len:usize=10,desc:"The number of chars.";
    }.parse_or_exit();

    let mut upper=String::from("");
    let mut lower=String::from("");
    let mut number=String::from("");
    let mut symbol=String::from("");

    if args.upper { upper = String::from("QWERTYUIOPASDFGHJKLZXCVBNM");};
    if args.lower { lower = String::from("qwertyuiopasdfghjklzxcvbnm");};
    if args.number { number = String::from("1234567890");};
    if args.symbol { symbol = String::from("!£$%&()=?^@#:;");};

    let mut total: String="".to_owned();
    total.push_str(&upper);
    total.push_str(&lower);
    total.push_str(&number);
    total.push_str(&symbol);
        
    if total.len()==0 {
        return;
    }

    let mut n = 0;
    let mut rng = rand::thread_rng();
    let mut result:String="".to_owned();
    while n < args.password_len {
        result.push(total.chars().nth(rng.gen_range(0..total.len()-1)).unwrap());
        n += 1;
    }

    println!("{}", result);
}
