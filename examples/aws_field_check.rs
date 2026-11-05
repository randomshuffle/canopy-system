//! Fail closed if an AWS timing binary would use portable field multiplication
//! or transcript instrumentation. Also checks the actual compiled kernel.
use canopy_system::binary_field::{mul128, mul128_reference, multiplication_backend};
fn main() {
    assert!(
        !cfg!(feature = "fs-accounting"),
        "timing binary has FS instrumentation"
    );
    let backend = multiplication_backend();
    if std::env::args().any(|x| x == "--require-intel" || x == "--require-pclmul") {
        assert_eq!(
            backend, "x86_64-pclmulqdq",
            "build on the x86 host with -C target-cpu=native"
        );
    }
    for i in 0..128 {
        for j in 0..128 {
            assert_eq!(mul128(1 << i, 1 << j), mul128_reference(1 << i, 1 << j));
        }
    }
    println!("{{\"field\":\"GF(2^256)\",\"backend\":\"{backend}\",\"basis_products_checked\":16384,\"fs_accounting\":false}}");
}
