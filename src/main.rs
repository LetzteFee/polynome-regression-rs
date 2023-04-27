struct Koeffizient {
    value: f64,
    sum: f64,
    expo: i32,
}
impl Koeffizient {
    fn new(e: i32) -> Koeffizient {
        Koeffizient {
            value: 0.0,
            sum: 1.0,
            expo: e,
        }
    }
    fn increase_value(&mut self) {
        self.value += self.sum;
    }
    fn decrease_value(&mut self) {
        self.value -= self.sum;
    }
    fn increase_sum(&mut self) {
        self.sum *= 2.0;
    }
    fn decrease_sum(&mut self) {
        self.sum /= 2.0;
        if self.sum == 0.0 {
            self.sum = 1.0;
        }
    }
}
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}
struct Polynom {
    koeffizienten: Vec<Koeffizient>,
    plot: Vec<Point>,
}
impl Polynom {
    fn new(grad: i32, plot: Vec<Point>) -> Polynom {
        let mut ke: Vec<Koeffizient> = Vec::new();
        for i in 0..(grad + 1) {
            ke.push(Koeffizient::new(i));
        }
        Polynom {
            koeffizienten: ke,
            plot: plot,
        }
    }
    fn f(&self, x: f64) -> f64 {
        let mut sum: f64 = 0.0;
        for i in &self.koeffizienten {
            sum += i.value * x.powf(f64::from(i.expo));
        }
        sum
    }
    fn delta(&self) -> f64 {
        let mut delta: f64 = 0.0;
        for i in &self.plot {
            let mut tmp: f64 = f64::from(i.y) - self.f(f64::from(i.x));
            tmp = tmp.abs();
            delta += tmp.powf(2.0);
        }

        delta
    }
    fn improve(&mut self) {
        for i in 0..self.koeffizienten.len() {
            let orig_delta: f64 = self.delta();
            let plus_delta: f64;
            let minus_delta: f64;

            self.koeffizienten[i].increase_value();
            plus_delta = self.delta();
            self.koeffizienten[i].decrease_value();

            self.koeffizienten[i].decrease_value();
            minus_delta = self.delta();
            self.koeffizienten[i].increase_value();

            if plus_delta < orig_delta && plus_delta < minus_delta {
                self.koeffizienten[i].increase_value();
                self.koeffizienten[i].increase_sum();
            } else if minus_delta < orig_delta {
                self.koeffizienten[i].decrease_value();
                self.koeffizienten[i].increase_sum();
            } else {
                self.koeffizienten[i].decrease_sum();
            }
        }
    }
    fn print(&self) -> String {
        let mut output: String = String::new();
        let mut tmp1: f64;
        let mut tmp2: i32;

        for i in 0..self.koeffizienten.len() {
            tmp1 = self.koeffizienten[i].value;
            tmp2 = self.koeffizienten[i].expo;
            output += &format!("+ {}*x^{} ", tmp1, tmp2);
        }
        output
    }
}
fn main() {
    let mut regression: Polynom = Polynom::new(
        3,
        vec![
            Point::new(1, -5),
            Point::new(20, -50),
            Point::new(50, -5),
            Point::new(100, -10),
        ],
    );
    let mut delta: f64 = regression.delta();
    for i in 0..5000 {
        for _j in 0..1000 {
            regression.improve();
        }

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
    println!("{}", regression.delta());
    println!("{}", regression.print());
}
