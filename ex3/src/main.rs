fn main() {
   
}

fn add(a:i32, b:i32) -> i32 {
    let suma = a+b;  
    return suma;
}

fn sub(a:i32,b:i32) -> i32 {
    let dif = a-b; 
    return dif;
}

fn mul(a:i32, b:i32) -> i32 {
    let prod = a*b;
    return prod;
}

fn div(a:i32, b:i32) -> Option<i32> {
    if b == 0 {
        return None;
    } else {
        return Some(a/b);
    }
}

fn rem(a:i32, b:i32) {
    if a < 0 {
        return a*(-1);
    } else return a; 
}