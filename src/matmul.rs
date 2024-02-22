pub(crate) fn dot(a: &[f64], b: &[f64]) -> f64 {
    // Function body here
    let mut c: f64 = 0.0;
    for i in 0..a.len() {
        c += a[i]*b[i];
    }
    c
}

