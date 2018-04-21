/* implements the chinese remainder theorem and utilities */
use euclidean;

/* special case in which there are only two numbers */
/* given N = a mod q and b mod r returns N mod qr  */
pub fn crt2numbers(a: i32, q: i32, b: i32, r:i32) -> Result<i32, String> {
    let (q1, r1, gcd) = euclidean::extended_euclidean(q, r);
    
    if gcd != 1 {
        return Err(String::from("Mods not co-prime"));
    }

    let result = (b * q * q1 + a * r * r1) % (q * r);
    Ok(result)
}

