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
}

