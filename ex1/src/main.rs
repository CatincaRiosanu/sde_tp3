fn main() {
    let variabila: Option<i32> = div(4, 2) ;

    match variabila {

        None => {
            println!("Nu se poate realiza impartirea!");
        }

        Some(variabila) => {
            println!("{}", variabila);
        }
    }
}

fn div (a: i32, b:i32) -> Option<i32> {
    if b == 0 {
        return None;
    } else {
        return Some(a/b);
    }
}