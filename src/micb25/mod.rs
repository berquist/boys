use core::{convert::TryFrom as _, f64::consts::PI};

use rgsl::{error::erf, Pow as _};

mod data;

#[inline]
#[must_use]
pub fn boys(n: u64, x: f64) -> f64 {
    let eps = 1.0e-14_f64;
    if n == 0 && x < eps {
        1.0
    } else if n == 0 {
        (PI / (4.0 * x)).sqrt() * erf(x.sqrt())
    } else if x < eps {
        #[expect(clippy::cast_precision_loss)]
        let n = n as f64;
        1.0 / 2.0_f64.mul_add(n, 1.0)
    } else if x > 50.0 {
        let ns = usize::try_from(n).unwrap();
        let n32 = u32::try_from(n).unwrap();
        N_FAC2_DBLE[2 * (ns - 1) + 2] / 2.0_f64.pow_uint(n32 + 1)
            * (PI / x.pow_uint(2 * n32 + 1)).sqrt()
    } else if x > 10.0 {
        #[expect(clippy::cast_possible_truncation)]
        #[expect(clippy::cast_sign_loss)]
        let j = ((x - 9.95) * 10.0).floor() as usize;
        let dx = data::BOYS_FUNC_VALUES_L[j][0] - x;
        let mut dxi = dx;
        let n = usize::try_from(n).unwrap();
        let mut lres = data::BOYS_FUNC_VALUES_L[j][n + 1];
        let epsrel = lres * eps;
        for (i, fac) in N_FAC_DBLE.iter().enumerate().take(MAX_RECURSION_DEPTH) {
            let sfac = data::BOYS_FUNC_VALUES_L[j][n + 2 + i] * dxi / fac;
            lres += sfac;
            if sfac.abs() < epsrel {
                return lres;
            }
            dxi *= dx;
        }
        lres
    } else if x > 5.0 {
        #[expect(clippy::cast_possible_truncation)]
        #[expect(clippy::cast_sign_loss)]
        let j = ((x - 4.975) * 20.0).floor() as usize;
        let dx = data::BOYS_FUNC_VALUES_M[j][0] - x;
        let mut dxi = dx;
        let n = usize::try_from(n).unwrap();
        let mut lres = data::BOYS_FUNC_VALUES_M[j][n + 1];
        let epsrel = lres * eps;
        for (i, fac) in N_FAC_DBLE.iter().enumerate().take(MAX_RECURSION_DEPTH) {
            let sfac = data::BOYS_FUNC_VALUES_M[j][n + 2 + i] * dxi / fac;
            lres += sfac;
            if sfac.abs() < epsrel {
                return lres;
            }
            dxi *= dx;
        }
        lres
    } else {
        #[expect(clippy::cast_possible_truncation)]
        #[expect(clippy::cast_sign_loss)]
        let j = x.mul_add(40.0, 0.5).floor() as usize;
        let dx = data::BOYS_FUNC_VALUES_S[j][0] - x;
        let mut dxi = dx;
        let n = usize::try_from(n).unwrap();
        let mut lres = data::BOYS_FUNC_VALUES_S[j][n + 1];
        let epsrel = lres * eps;
        for (i, fac) in N_FAC_DBLE.iter().enumerate().take(MAX_RECURSION_DEPTH) {
            let sfac = data::BOYS_FUNC_VALUES_S[j][n + 2 + i] * dxi / fac;
            lres += sfac;
            if sfac.abs() < epsrel {
                return lres;
            }
            dxi *= dx;
        }
        lres
    }
}

#[cfg(test)]
mod tests {
    use super::boys;
    use std::fs;

    #[test]
    fn boys_works() {
        //! Recreate the table from
        //! https://github.com/micb25/libboys/blob/7d7cb7951d48da3db912b14dbb47e2f6edbd5c85/TESTING/boys_test.f90
        let data = fs::read_to_string("./benchmark_values.txt").expect("unable to read file");

        println!("-----------------------------------------------------------------------------------------------");
        println!(
            "{:>5} {:>22} {:>22} {:>22} {:>22}",
            "N", "X", "calc.value", "ref. value", "diff"
        );
        println!("-----------------------------------------------------------------------------------------------");
        for line in data.lines() {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            // Allow comments
            if tokens[0] != "#" {
                let n: u64 = tokens[0].parse().unwrap();
                let t: f64 = tokens[1].parse().unwrap();
                let m: f64 = tokens[2].parse().unwrap();
                let res = boys(n, t);
                println!(
                    "{:5} {:22.16} {:22.16} {:22.16} {:22.16}",
                    n,
                    t,
                    res,
                    m,
                    res - m
                );
                assert!((res - m).abs() < 1.0e-12);
            }
        }
        println!("-----------------------------------------------------------------------------------------------");
    }
}

/// Level of summation to perform
const MAX_RECURSION_DEPTH: usize = 6;

/// Precomputed values of factorial
const N_FAC_DBLE: [f64; 6] = [1.0, 2.0, 6.0, 24.0, 120.0, 720.0];

/// Precomputed values of double factorial
const N_FAC2_DBLE: [f64; 31] = [
    1.0,
    1.0,
    1.0,
    2.0,
    3.0,
    8.0,
    15.0,
    48.0,
    105.0,
    384.0,
    945.0,
    3_840.0,
    10_395.0,
    46_080.0,
    135_135.0,
    645_120.0,
    2_027_025.0,
    10_321_920.0,
    34_459_425.0,
    185_794_560.0,
    654_729_075.0,
    3_715_891_200.0,
    13_749_310_575.0,
    81_749_606_400.0,
    316_234_143_225.0,
    1_961_990_553_600.0,
    7_905_853_580_625.0,
    51_011_754_393_600.0,
    213_458_046_676_875.0,
    1_428_329_123_020_800.0,
    6_190_283_353_629_375.0,
];
