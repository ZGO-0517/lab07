use rand::RngExt;
const DAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

fn main() {
    let mut highs: Vec<i32> = vec![72, 68, 75, 81, 79];

    let mut rng = rand::rng();

    highs.push(rng.random_range(60..=100));
    highs.push(rng.random_range(60..=100));

	for (day, high) in DAYS.iter().zip(highs.iter()) {
	println!("{}: {}", day, high);
	}

    println!("Average of highs is: {}", average_temp(&highs));
    println!("The hottest day is: {}", hottest_day(&highs));
    println!("{} days go above the threshold(75)", count_above(&highs, 75));

}

fn average_temp(log: &Vec<i32>) -> f64 {
    if log.is_empty() {
        return 0.0;
    }

    let sum: i32 = log.iter().sum();

    sum as f64 / log.len() as f64
}

fn hottest_day(log: &Vec<i32>) -> usize {
	let mut sum: i32 = 0;
	for high in log {
		if *high > sum {
			sum = *high
	 		}
		}
	sum.try_into().unwrap()
}

fn count_above(log: &Vec<i32>, threshold: i32) -> usize {
	let mut sum: i32 = 0;
	for high in log {
		if *high > threshold {
			sum += 1;
		}
	}
	sum.try_into().unwrap()
}
