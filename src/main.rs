// pi = 4(1 - 1/3 + 1/5 - 1/7 + 1/9 - 1/11 + ....)

struct PiCalculator {
    total: f64,
    prev_total: f64,
    tolerance: f64,
    num_terms: usize
}

impl PiCalculator {
    fn new(tolerance: f64) -> Self {
        PiCalculator { total: 1.0, prev_total: 0.0, num_terms: 1, tolerance }
    }

    fn current_total(&self) -> f64 {
        self.total * 4.0
    }

    fn get_series_term(&self) -> f64 {
        let sign = if self.num_terms % 2 == 0 {1.0} else {-1.0};
        let denominator = 2 * self.num_terms + 1;
        sign * 1.0 / denominator as f64
    }

    fn update(&mut self) {
        self.prev_total = self.total;
        self.total += self.get_series_term();
        self.num_terms += 1;
    }

    fn done(&self) -> bool {
        (self.total - self.prev_total).abs() < self.tolerance
    }
}

fn main() {
    let tolerance = std::env::args().skip(1).next().unwrap().parse::<f64>().unwrap();
    let mut calculator = PiCalculator::new(tolerance);
    while !calculator.done() {
        calculator.update();
    }
    println!("Pi: {}", calculator.current_total());
}
