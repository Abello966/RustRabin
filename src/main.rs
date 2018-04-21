extern crate rustrabin;
use std::io;

fn readnum() -> Result<i32, std::num::ParseIntError> {
    let mut read = String::new();
    io::stdin().read_line(&mut read)
               .expect("WTF BOOOM");
    return read.trim().parse();
}

fn main() {
    loop {
        println!("Print a > b integers for mdc");
        
        let a = match readnum() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let b = match readnum() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let (u, v, gdc) = rustrabin::euclidean::extended_euclidean(a, b);
        println!("GDC is {}", gdc);
        println!("{} = {} * {} + {} * {}", gdc, u, a, v, b);

        
        if gdc == u * a + b * v {
            println!("It works!");
        }
        else {
            println!("It doesnt work...");
        }
       

    }
}
