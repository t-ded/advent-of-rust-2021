const W_DAY: usize = 10;
const W_PART: usize = 10;

fn mean(numbers: &[f64]) -> f64 {
    numbers.iter().sum::<f64>() / numbers.len() as f64
}

fn measure_run<S: ?Sized, T, F: Fn(&S) -> T>(f: &F, input: &S) -> f64 {
    let start = std::time::SystemTime::now();
    _ = f(input);
    let duration = start.elapsed().unwrap();
    duration.as_secs_f64()
}

pub fn benchmark_run<S: ?Sized, T, F: Fn(&S) -> T>(f: F, input: &S) -> f64 {
    let first_run = measure_run(&f, input);
    let n = (1. / first_run) as i32;
    if n <= 1 || first_run < 0.000001 {
        return first_run;
    }
    let mut run_times = vec![];
    for _ in 0..n {
        run_times.push(measure_run(&f, input));
    }
    mean(&run_times)
}

pub fn print_header() {
    print!("{:<w$}", "day", w = W_DAY);
    print!("{:<w$}", "part 1", w = W_PART);
    print!("{:<w$}", "part 2", w = W_PART);
    println!();
    println!("{:-<w$}", "", w = W_DAY + W_PART * 2);
}

pub fn print_day(day: u8, p1: f64, p2: f64) {
    print!("{:<w$}", format!("day {:02}", day), w = W_DAY);

    let mut p1_dur = format!("{:.3}", p1 * 1000.).to_string();
    p1_dur = format!("{} ms", &p1_dur[..5]);
    print!("{:<w$}", p1_dur, w = W_PART);

    let mut p2_dur = format!("{:.3}", p2 * 1000.).to_string();
    p2_dur = format!("{} ms", &p2_dur[..5]);
    println!("{:<w$}", p2_dur, w = W_PART);
}

#[macro_export]
macro_rules! benchmark_all {
    ($($day:ident),*) => {{
        print_header();
        $(
        let day_number_string = &stringify!($day).to_string()[4..];
        let input_path = format!("inputs/{}.in", day_number_string);
        let error_msg = format!("Unable to open input file {}", &input_path);
        let raw_input = std::fs::read_to_string(&input_path).expect(error_msg.as_str());

        let p1_duration = benchmark_run($day::part_1, &raw_input);
        let p2_duration = benchmark_run($day::part_2, &raw_input);

        print_day(day_number_string.parse().unwrap(), p1_duration, p2_duration);
        )*
    }};
}