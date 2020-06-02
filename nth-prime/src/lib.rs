pub fn nth(n: u32) -> u32 {
    let mut counter = 0;
    let mut i = 1;

    while counter <= n {
        i += 1;
        if is_prime(i) {
            counter += 1;
        }
    }

    i
}

fn is_prime(n: u32) -> bool {
    let mut i = 2;

    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1
    }
    true
}
