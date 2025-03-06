#[test]
fn api() {
    let thresh = 1.0e-16;
    let ref_2_2p0 = 0.0529428148329765;
    assert!(boys::exact::boys(2, 2.0) - ref_2_2p0 < thresh);
    assert!(boys::micb25::boys(2, 2.0) - ref_2_2p0 < thresh);
}
