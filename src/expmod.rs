/* Implements modular exponential and utilities */

//exponentiate m to the power of e mod n efficiently
pub fn expmod(m: i32, ein: i32, n: i32) -> (i32) {
    let mut base = m;
    let mut temp = 1;
    let mut e = ein;

    while e > 0 {
        if e % 2 == 1 {
            temp = (temp * base) % n;
        }
        base = (base * base) % n;
        e = e / 2;
    }
    temp
}

