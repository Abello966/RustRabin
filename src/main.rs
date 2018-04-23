extern crate rustrabin;

fn main() {
     
    //test rabin cryptosystem
    println!("Rabin Cryptosystem");
    let q = 31;
    let r = 7;
    let n = q * r;
    let x = 22;
    let y = rustrabin::rabin::encrypt(x, n, true); 
    let _cands = rustrabin::rabin::decrypt(y, q, r, true);

    println!("ElGamal Signature");
    //Quick ElGamal Signature implementation
    let p = 7;
    let g = 3;
    let S = 5;
    let T = 5;
    let x = 4;
    let k = 5;

    //sign
    println!("k inverse mod p -1");
    let k1 = rustrabin::euclidean::inverse_mod_n(k, p - 1, true).unwrap();
    println!("k1: {}", k1);
    println!("Signing: y");
    let y = rustrabin::expmod::expmod(g, k, p, true);
    println!("Signing: z");
    let z = x - S * y;
    println!("x - Sy: {}", z);
    let z = z * k1  % (p - 1);
    let z = (z + (p - 1)) % (p - 1);
    println!("z: {}", z);

    println!("Verifying Signature");
    println!("(T ** y) mod p");
    let ty = rustrabin::expmod::expmod(T, y, p, true);
    println!("(y ** z) mod p");
    let yz = rustrabin::expmod::expmod(y, z, p, true);
    println!("(g ** x) mod p");
    let gx = rustrabin::expmod::expmod(g, x, p, true);

    if y >= 1 && y <= (p - 1) {
        //go on
        let lh = (ty * yz ) % p;
        let lh = (lh + p) % p;
        if lh == gx {
            println!("Signature is correct");
        }
        else {
            println!("ty yz =/= gx mod p");
        }
    }
    else {
        println!("y not in Z(p-1)");
    }
}
