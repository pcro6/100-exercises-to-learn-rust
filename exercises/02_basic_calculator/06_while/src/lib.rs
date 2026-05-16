// Rewrite the factorial function using a `while` loop.
pub fn factorial(n: u32) -> u32 {

    let mut f:u32 = 1;
    let mut i:u32 = 0;

    while i < n {
        i = i+1;
        f = f*i;
    }
    return f
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
