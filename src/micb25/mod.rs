use std::f64::consts::PI;

use rgsl::error::erf;

mod data;

pub fn boys(n: u64, x: f64) -> f64 {
    let eps = 1.0e-14;
    let max_recursion_depth = 6;
    if n == 0 && x < eps {
        1.0
    } else if n == 0 {
        (PI / (4.0 * x)).sqrt() * erf(x.sqrt())
    } else if x < eps {
        1.0 / ((2.0 * n as f64) + 1.0)
    } else if x > 50.0 {
        N_FAC2_DBLE[(2 * (n - 1) + 2) as usize] / 2.0_f64.powi(n as i32 + 1)
            * (PI / x.powi(2 * n as i32 + 1)).sqrt()
    } else if x > 10.0 {
        let j = ((x - 9.95) * 10.0) as usize;
        let dx = data::BOYS_FUNC_VALUES_L[j][0] - x as f64;
        let mut dxi = dx;
        let mut lres = data::BOYS_FUNC_VALUES_L[j][n as usize + 1];
        let epsrel = lres * eps;
        for i in 0..max_recursion_depth {
            let sfac = data::BOYS_FUNC_VALUES_L[j][n as usize + 2 + i] * dxi / N_FAC_DBLE[i];
            lres += sfac;
            if sfac.abs() < epsrel {
                return lres;
            }
            dxi *= dx;
        }
        lres
    } else if x > 5.0 {
        let j = ((x - 4.975) * 20.0) as usize;
        let dx = data::BOYS_FUNC_VALUES_M[j][0] - x as f64;
        let mut dxi = dx;
        let mut lres = data::BOYS_FUNC_VALUES_M[j][n as usize + 1];
        let epsrel = lres * eps;
        for i in 0..max_recursion_depth {
            let sfac = data::BOYS_FUNC_VALUES_M[j][n as usize + 2 + i] * dxi / N_FAC_DBLE[i];
            lres += sfac;
            if sfac.abs() < epsrel {
                return lres;
            }
            dxi *= dx;
        }
        lres
    } else {
        let j = ((x * 40.0) + 0.5) as usize;
        let dx = data::BOYS_FUNC_VALUES_S[j][0] - x as f64;
        let mut dxi = dx;
        let mut lres = data::BOYS_FUNC_VALUES_S[j][n as usize + 1];
        let epsrel = lres * eps;
        for i in 0..max_recursion_depth {
            let sfac = data::BOYS_FUNC_VALUES_S[j][n as usize + 2 + i] * dxi / N_FAC_DBLE[i];
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
    fn test_boys() {
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

const N_FAC_DBLE: [f64; 6] = [1.0, 2.0, 6.0, 24.0, 120.0, 720.0];

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
