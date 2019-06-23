use std::f64::consts::PI;

fn inverse_factorial(n: u64) -> f64 {
    let mut r = 1.0;
    for i in 1..(n + 1) {
        r /= i as f64;
    }
    r
}

fn double_factorial(n: u64) -> u64 {
    let mut r = 1;
    for i in 1..(n + 1) {
        r *= 2 * i - 1;
    }
    r
}

fn boys_asymp(n: u64, x: f64) -> f64 {
    double_factorial(2 * n - 1) as f64 / 2.0_f64.powf(n as f64 + 1.0)
        * (PI / x.powf(2.0 * n as f64 + 1.0))
}

fn boys_term(n: u64, k: u64, x: f64) -> f64 {
    -x.powf(k as f64) * inverse_factorial(k) / ((2.0 * (k as f64)) + (2.0 * (n as f64)) + 1.0)
}

pub fn boys(n: u64, x: f64) -> f64 {
    let mut k = 0;
    let mut b = boys_term(n, k, x);
    let mut s = 0.0;
    println!("===================================================");
    if x < 1.0 {
        println!(
            "{:>3} {:>3} {:>10} {:>14} {:>14} {:>14} {:>14}",
            "n", "k", "x", "k!", "x^k", "BoysTerm(n,k,x)", "sum"
        );
    } else {
        println!(
            "{:>3} {:>3} {:>10} {:>14} {:>14} {:>14} {:>14} {:>14}",
            "n", "k", "x", "k!", "x^k", "BoysTerm(n,k,x)", "sum", "BoysAsymp(n, x)"
        );
    }
    while (k < 200) && (b.abs() > 1.0e-16) {
        s += b;
        if x < 10.0 {
            println!(
                "{:>3} {:>3} {:>10.5} {:>14.7e} {:>14.7e} {:>14.7e} {:>14.7e} {:>3} {:>1}",
                n,
                k,
                x,
                inverse_factorial(k),
                x.powf(k as f64),
                b,
                s,
                k + 1,
                if b.abs() < 1.0e-14 { '*' } else { ' ' }
            );
        } else {
            println!(
                "{:>3} {:>3} {:>10.5} {:>14.7e} {:>14.7e} {:>14.7e} {:>14.7e} {:>14.7e} {:>3} {:>1} {:>2}",
                n,
                k,
                x,
                inverse_factorial(k),
                x.powf(k as f64),
                b,
                s,
                boys_asymp(n, x),
                k + 1,
                if b.abs() < 1.0e-14 { '*' } else { ' ' },
                if (boys_asymp(n, x) - s).abs() < 1.0e-10 {
                    "**"
                } else {
                    "  "
                }
            );
        }
        k += 1;
        b = boys_term(n, k, x);
    }
    println!("===================================================");
    println!(
        "{:>4} {:>2} {:>10} {:>12} {:>12} {:>14} {:>14} {:>14} {:>14} {:>14} {:>14} {:>14} {:>14}",
        "n",
        "k",
        "x",
        "2n+2k+1",
        "fast 2n+2k+1",
        "1/k!",
        "fast 1/k!",
        "x^k",
        "fast x^k",
        "BoystTerm",
        "BoysTermFast",
        "sum",
        "fast sum"
    );
    k = 0;
    let mut knsum = 2.0 * n as f64 + 1.0;
    let mut invkfact = 1.0;
    let mut xpower = 1.0;
    let mut bterm_even = 1.0 / knsum;
    let mut bterm_odd: f64;
    let mut sum = boys_term(n, k, x);
    let mut fast = 1.0 / knsum;
    println!(
        "{:>4} {:>2} {:>10.5} {:>12.6e} {:>12.6e} {:>14.7e} {:>14.7e} {:>14.7e} {:>14.7e} {:>14.7e} {:>14.7e} {:>14.7e} {:>14.7e}",
        n,
        k,
        x,
        2.0 * n as f64 + 2.0 * k as f64 + 1.0,
        knsum,
        inverse_factorial(k),
        invkfact,
        x.powf(k as f64),
        xpower,
        boys_term(n, k, x),
        bterm_even,
        sum,
        fast
    );
    while k < 21 {
        k += 1;
        sum += boys_term(n, k, x);
        xpower *= x;
        invkfact /= k as f64;
        knsum += 2.0;
        bterm_odd = -xpower * invkfact / knsum;
        fast += bterm_odd;
        println!(
            "{:>4} {:>2} {:>10.5} {:>12.6e} {:>12.6e} {:>14.7e} {:>14.7e} {:>14.7e} {:>14.7e} {:>14.7e} {:>14.7e} {:>14.7e} {:>14.7e}",
            n,
            k,
            x,
            2.0 * n as f64 + 2.0 * k as f64 + 1.0,
            knsum,
            inverse_factorial(k),
            invkfact,
            x.powf(k as f64),
            xpower,
            boys_term(n, k, x),
            bterm_odd,
            sum,
            fast
    );
        k += 1;
        sum += boys_term(n, k, x);
        xpower *= x;
        invkfact /= k as f64;
        knsum += 2.0;
        bterm_even = xpower * invkfact / knsum;
        fast += bterm_even;
        println!(
            "{:>4} {:>2} {:>10.5} {:>12.6e} {:>12.6e} {:>14.7e} {:>14.7e} {:>14.7e} {:>14.7e} {:>14.7e} {:>14.7e} {:>14.7e} {:>14.7e}",
            n,
            k,
            x,
            2.0 * n as f64 + 2.0 * k as f64 + 1.0,
            knsum,
            inverse_factorial(k),
            invkfact,
            x.powf(k as f64),
            xpower,
            boys_term(n, k, x),
            bterm_even,
            sum,
            fast
        );
    }
    println!("===================================================");
    println!("{:>4} {:>20} {:>20}", "k", "term", "sum");
    println!("===================================================");
    k = 0;
    xpower = 1.0;
    invkfact = 1.0;
    knsum = 2.0 * n as f64 + 1.0;
    bterm_even = 1.0 / knsum;
    let len: u64 = 1000;
    let mut fast_vec = Vec::with_capacity(len as usize);
    fast_vec.push(bterm_even);
    while k < len {
        k += 1;
        xpower *= x;
        invkfact /= k as f64;
        knsum += 2.0;
        fast_vec.push(-xpower * invkfact / knsum);
        k += 1;
        xpower *= x;
        invkfact /= k as f64;
        knsum += 2.0;
        fast_vec.push(xpower * invkfact / knsum);
    }
    fast = 0.0;
    k = 0;
    while k < len && fast_vec[k as usize].abs() > 1.0e-14 {
        fast += fast_vec[k as usize];
        println!("{:>4} {:>24.14e} {:>24.14e}", k, fast_vec[k as usize], fast);
        k += 1;
    }
    println!("===================================================");
    s
}

#[cfg(test)]
mod tests {
    use super::boys;
    use super::inverse_factorial;

    #[test]
    fn test_inverse_factorial() {
        assert_eq!(inverse_factorial(0), 1.0);
        assert_eq!(inverse_factorial(1), 1.0 / 1.0);
        assert_eq!(inverse_factorial(2), 1.0 / 1.0 / 2.0);
        assert_eq!(inverse_factorial(3), 1.0 / 1.0 / 2.0 / 3.0);
    }

    #[test]
    fn test_boys() {
        boys(2, 2.0);
        // boys(14, 42.67768466983068);
    }
}
