struct Koeffizient {
    value: f64,
    orig_value: f64,
    sum: f64,
    expo: i32,
}
impl Koeffizient {
    fn new(expo: i32) -> Koeffizient {
        Koeffizient {
            value: 0.0,
            orig_value: 0.0,
            sum: 1.0,
            expo,
        }
    }
    fn increase_value(&mut self) {
        self.save_value();
        self.value += self.sum;
    }
    fn decrease_value(&mut self) {
        self.save_value();
        self.value -= self.sum;
    }
    fn reset_value(&mut self) {
        self.value = self.orig_value;
    }
    fn save_value(&mut self) {
        self.orig_value = self.value;
    }
    fn increase_sum(&mut self) {
        self.sum *= 2.0;
    }
    fn decrease_sum(&mut self) {
        self.sum /= 2.0;
        if self.sum == 0.0 {
            self.reset_sum();
        }
    }
    fn reset_sum(&mut self) {
        self.sum = 1.0;
    }
}
pub struct Point {
    x: i32,
    y: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}
pub struct Polynom {
    koeffizienten: Vec<Koeffizient>,
    plot: Vec<Point>,
    delta_expo: i32,
    pub calculations_per_call: u32,
}
impl Polynom {
    pub fn new(grad: i32, plot: Vec<Point>) -> Polynom {
        let mut koeffizienten: Vec<Koeffizient> = Vec::new();
        for i in 0..(grad + 1) {
            koeffizienten.push(Koeffizient::new(i));
        }
        Polynom {
            koeffizienten,
            plot,
            delta_expo: 2,
            calculations_per_call: 20000,
        }
    }
    fn f(&self, x: f64) -> f64 {
        let mut sum: f64 = 0.0;
        for ko in &self.koeffizienten {
            sum += ko.value * x.powi(ko.expo);
        }
        sum
    }
    pub fn delta(&self) -> f64 {
        let mut delta: f64 = 0.0;
        for i in &self.plot {
            delta += (f64::from(i.y) - self.f(f64::from(i.x)))
                .abs()
                .powi(self.delta_expo);
        }
        delta
    }
    pub fn improve(&mut self) {
        for _j in 0..self.calculations_per_call {
            for i in 0..self.koeffizienten.len() {
                let orig_delta: f64 = self.delta();
                let plus_delta: f64;
                let minus_delta: f64;

                self.koeffizienten[i].increase_value();
                plus_delta = self.delta();
                self.koeffizienten[i].reset_value();

                self.koeffizienten[i].decrease_value();
                minus_delta = self.delta();
                self.koeffizienten[i].reset_value();

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
    }
    pub fn print(&self) -> String {
        let mut output: String = String::new();
        for i in &self.koeffizienten {
            output += &format!("+ {}*x^{} ", i.value, i.expo);
        }
        output
    }
}
