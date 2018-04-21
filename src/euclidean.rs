/* implements extended euclidean algorithm and utilities */

/* calculates the gcd and u, v such that gcd(a, b) = u * a + v * b */
/* returns u, v, gcd */
pub fn extended_euclidean(a: i32, b: i32) -> (i32, i32, i32) {
    /* First iteration: a = 1a + 0b */
    let mut u_2: i32 = 1;
    let mut v_2: i32 = 0;
    
    /* b = 0a + 1b */
    let mut u_1: i32 = 0;
    let mut v_1: i32 = 1;
    
    let mut x_2: i32 = a;
    let mut x_1: i32 = b;
    let mut x = x_2 % x_1;
    
    while x > 0 {
        /* calculate new coefficients */
        let q = x_2 / x_1;
        let u_0 = u_2 - q * u_1;
        let v_0 = v_2 - q * v_1;

        /* update them */
        u_2 = u_1;
        u_1 = u_0;
    
        v_2 = v_1;
        v_1 = v_0;
    
        x_2 = x_1;
        x_1 = x;
    
        x = x_2 % x_1;
    }
    
    return (u_1, v_1, x_1);
}

/*using the algorithm above we can calculate the inverse mod n */
pub fn inverse_mod_n(a: i32, n:i32) -> Result<i32, String> {
    let (_, v, mdc) = extended_euclidean(n, a);
    return match mdc {
        1 => Ok(v % n),
        _ => Err(String::from("Arguments were not co-prime")),
    };
}
