use rand::RngExt;
const DAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

struct Reading {
    day: String,
    high: i32,
}

struct Summary {
    average: f64,
    hottest_day: String,
    days_above: usize,
}

fn main() {
    let mut highs: Vec<i32> = vec![72, 68, 75, 81, 79];

    let mut rng = rand::rng();

    highs.push(rng.random_range(60..=100));
    highs.push(rng.random_range(60..=100));

    let mut log: Vec<Reading> = Vec::new();

	for (day, high) in DAYS.iter().zip(highs.iter()) {
		log.push(Reading {
			day: day.to_string(),
			high: *high,
	});
}

for reading in &log {
	println!("{}: {}", reading.day, reading.high);
}

let summary = summarize(&log, 75);

    println!("Average of highs is: {}", summary.average);
    println!("The hottest day is: {}", summary.hottest_day);
    println!("{} days go above the threshold(75)", summary.days_above);

}

fn average_temp(log: &Vec<Reading>) -> f64 {
    if log.is_empty() {
        return 0.0;
    }

    let sum: i32 = log.iter().map(|r| r.high).sum();

    sum as f64 / log.len() as f64
}

fn hottest_day(log: &Vec<Reading>) -> usize {
	if log.is_empty() {
		return 0;
	}

	let mut max_idx = 0;
	let mut max_temp = log[0].high;
	for i in 1..log.len() {
		if log[i].high > max_temp {
			max_temp = log[i].high;
			max_idx = i;
	 		}
		}
	max_idx
}

fn count_above(log: &Vec<Reading>, threshold: i32) -> usize {
	let mut count = 0;
	for reading in log {
		if reading.high > threshold {
			count += 1;
		}
	}
	count
}

fn summarize(log: &Vec<Reading>, threshold: i32) -> Summary {
	let average = average_temp(log);
	let count = count_above(log, threshold);

	let hottest_label = if log.is_empty() {
		"None".to_string()
	}
	else {
		log[hottest_day(log)].day.clone()
	};

	Summary {
		average,
		hottest_day: hottest_label,
		days_above: count,
	}
}
