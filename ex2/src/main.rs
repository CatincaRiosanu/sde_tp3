use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

   // let var1 = args[1];
   // let var2 = args[2]; Nu asa fiindca args este string si mai jos am facut convert from str to int 
    
    if args.len() != 3 {
        if args.len() < 3 {
            println!("Au fost introduse prea putine argumente");
            std::process::exit(-1);
        } else if args.len() > 3 {
            println!("Au fost introduse prea multe argumente");
            std::process::exit(-1);
        }
    }

    let var1 = match args[1].parse::<i32>() {
        Ok(var1) => var1,
        Err(_) => std::process::exit(-1)
    };

    let var2 = match args[2].parse::<i32>() {
        Ok(var2) => var2,
        Err(_) => std::process::exit(-1)
    };

    match div(var1,var2) {
        None => {
            println!("Nu se poate realiza impartirea!");
        }

        Some(v) => {
            println!("{}", v);
        }
    }
}

fn div (a:i32, b:i32) -> Option<i32> {
    if b == 0 {
        return None;
    } else {
        return Some(a/b);
    }
}
