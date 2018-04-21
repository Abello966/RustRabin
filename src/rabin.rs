//Implements Rabin Criptosystem
use expmod;
use crt; 

//Produces cyphertext y from public key
pub fn encrypt(x: i64, n: i64) -> i64 {
    (x * x) % n
}

//decrypts cyphertext using private keys
//particular case in which 4 | q + 1, 4 | r + 1
//finds 4 candidates for plaintext
pub fn decrypt(y: i64, q: i64, r: i64) -> Vec<i64> {
    let mut ans: Vec<i64> = Vec::new();

    let x1 = expmod::expmod(y, (q + 1) / 4, q);
    let x2 = expmod::expmod(y, (r + 1) / 4, r);

    ans.push(crt::crt2numbers(x1, q, x2, r).unwrap());
    ans.push(crt::crt2numbers(-1 * x1, q, x2, r).unwrap());
    ans.push(crt::crt2numbers(x1, q, -1 * x2, r).unwrap());
    ans.push(crt::crt2numbers(-1 * x1, q, -1 * x2, r).unwrap());
    ans    
}
