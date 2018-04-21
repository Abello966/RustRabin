//Implements Rabin Criptosystem
use expmod;
use crt; 

//Produces cyphertext y from public key
pub fn encrypt(x: i32, n: i32) -> i32 {
    (x * x) % n
}

//decrypts cyphertext using private keys
//particular case in which 4 | q + 1, 4 | r + 1
//finds 4 candidates for plaintext
pub fn decrypt(y: i32, q: i32, r: i32) -> Vec<i32> {
    let mut ans: Vec<i32> = Vec::new();

    let x1 = expmod::expmod(y, (q + 1) / 4, q);
    let x2 = expmod::expmod(y, (r + 1) / 4, r);

    ans.push(crt::crt2numbers(x1, q, x2, r).unwrap());
    ans.push(crt::crt2numbers(-1 * x1, q, x2, r).unwrap());
    ans.push(crt::crt2numbers(x1, q, -1 * x2, r).unwrap());
    ans.push(crt::crt2numbers(-1 * x1, q, -1 * x2, r).unwrap());
    ans    
}
