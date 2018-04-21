pub mod crt;
pub mod euclidean;
pub mod expmod;
pub mod rabin;

#[cfg(test)]
mod tests {
    use euclidean;
    use crt;
    use expmod;
    use rabin;

    #[test]
    //euclidean tests
    fn euclidean_logic_test() {
        let a = 7;
        let b = 23;
        let (u, v, gcd) = euclidean::extended_euclidean(a, b, false);
        assert_eq!(gcd, u * a + b * v);
    }

    #[test]
    fn euclidean_correctness() {
        let a = 2520;
        let b = 1848;
        let (_a, _b, gcd) = euclidean::extended_euclidean(a, b, false);
        assert_eq!(gcd, 168);
    }

    //expmod test
    #[test]
    fn expmod_correctness() {
        assert_eq!(expmod::expmod(2, 3, 10, false), expmod::expmod(2, 3, 100, false));
        assert_eq!(expmod::expmod(2, 3, 10, false), 8);
        assert_eq!(expmod::expmod(31, 12, 23, false), 8);
    }

    //crt test
    #[test]
    fn crt_correctness() {
        assert_eq!(crt::crt2numbers(4, 7, 3, 11, false).unwrap(), 25);
    }

    //rabin cryptossystem tests
    #[test]
    fn rabin_logic() {
        let q = 31;
        let r = 7;
        let n = q * r;
        let x = 22;
        let y = rabin::encrypt(x, n, false);
        let cands = rabin::decrypt(y, q, r, false);
        let mut isin = false;
        for cand in cands {
            if cand == x {
                isin = true
            }
        }
        assert!(isin);
    }

    #[test]
    fn rabin_correctness() {
        let q = 1123;
        let r = 7723;
        let n = q * r;
        let x = 4121313;
        assert_eq!(rabin::encrypt(x, n, false), 577647); 
    }
}



