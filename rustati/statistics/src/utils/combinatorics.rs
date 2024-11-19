pub fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

pub fn binomial_coeff(n: u64, k: u64) -> u64 {
    match k {
        k if k > n => 0,
        k if k == 0 || k == n => 1,
        _ => {
            let k = k.min(n - k);
            (0..k)
                .fold(1, |result, i| result * (n - i) / (i + 1))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_5() {
        let received = factorial(5);
        let expected = 120;
        assert_eq!(received, expected);
    }

    #[test]
    fn binomial_coeff_5_0() {
        let received = binomial_coeff(5, 0);
        let expected = 1;
        assert_eq!(received, expected);
    }

    #[test]
    fn binomial_coeff_5_5() {
        let received = binomial_coeff(5, 0);
        let expected = 1;
        assert_eq!(received, expected);
    }

    #[test]
    fn binomial_coeff_5_3() {
        let received = binomial_coeff(5, 3);
        let expected = 10;
        assert_eq!(received, expected);
    }
}
