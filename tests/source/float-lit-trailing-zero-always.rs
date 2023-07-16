// rustfmt-float_literal_trailing_zero: Always

fn float_literals() {
    let a = 0.;
    let b = 0.0;
    let c = 100.;
    let d = 100.0;
    let e = 5e3;
    let f = 5.0e3;
    let g = 7f32;
    let h = 7.0f32;
    let i = 9e3f32;
    let j = 9.0e3f32;
    let k = 1000.00;
    let l = 1_000_.;
    let m = 1_000_.000_000;
}
