extern crate rustrabin;
use std::io;

fn readnum() -> Result<i32, std::num::ParseIntError> {
    let mut read = String::new();
    io::stdin().read_line(&mut read)
               .expect("WTF BOOOM");
    return read.trim().parse();
}

fn main() {
    //euclidean tests
    /*
    loop {
        println!("Input a > b integers for mdc");
        
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
    */
    //expmod test
    /* 
    loop {
        println!("Input a, e, n integers for a to the power of e mod n");
        let a = match readnum() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let e = match readnum() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let n = match readnum() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("({} ** {}) % {} = {}", a, e, n, rustrabin::expmod::expmod(a, e, n));
    }*/
  
    //crt test 4 mod 7, 3 mod 11 => 25 mod 77
    /* 
    loop {
        println!("Input a mod q, b mod r for crt test");
        let a = match readnum() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let q = match readnum() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let b = match readnum() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let r = match readnum() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match rustrabin::crt::crt2numbers(a, q, b, r) {
            Ok(num) => println!("Ans: {}", num),
            Err(msg) => println!("{}", msg),
        };
    }
    */
    
    //test rabin cryptosystem
    let q = 1123;
    let r = 7723;
    let n = q * r;
    let x = 4121313;
    let y = 577647; 
    let cands = rustrabin::rabin::decrypt(y, q, r);
    println!("Encrypting {}: {}", x, y);
    println!("Decrypting. Candidates:");
    for cand in cands {
        println!("{}", cand);
    }

}
