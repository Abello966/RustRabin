extern crate rustrabin;

fn main() {
     
    //test rabin cryptosystem
    let q = 31;
    let r = 7;
    let n = q * r;
    let x = 22;
    let y = rustrabin::rabin::encrypt(x, n, true); 
    let _cands = rustrabin::rabin::decrypt(y, q, r, true);
}
