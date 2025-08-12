use boys::{exact as boys_exact, micb25 as boys_micb25};

#[test]
fn api() {
    let thresh = 1.0e-16_f64;
    let ref_2_2p0 = 0.052_942_814_832_976_5_f64;
    assert!(boys_exact::boys(2, 2.0) - ref_2_2p0 < thresh);
    assert!(boys_micb25::boys(2, 2.0) - ref_2_2p0 < thresh);
}
