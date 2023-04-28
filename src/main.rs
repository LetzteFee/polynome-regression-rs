mod regression_mod;
fn main() {
    let mut regression = regression_mod::Polynom::new(
        3,
        vec![
            regression_mod::Point::new(1, -5),
            regression_mod::Point::new(20, -50),
            regression_mod::Point::new(50, -5),
            regression_mod::Point::new(10000, -10),
        ],
    );
    regression.calculations_per_call = 30000;
    let mut delta: f64 = regression.delta();
    for i in 0..10 {
        regression.improve();
        if regression.delta() < delta {
            delta = regression.delta();
            println!("{}", delta);
            println!("{}", regression.print());
        }
        if delta == 0.0 {
            println!("Stopped at {}", i);
            break;
        }
    }
}
