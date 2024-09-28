use std::io;

fn main() {
	let mut nums = String::new();
    io::stdin()
		.read_line(&mut nums)
		.expect("Failed to read line");

	let nums = nums.trim().split_whitespace();
	let mut v: Vec<i32> = Vec::new();	

	for i in nums {
		v.push(i.parse().expect("Not a number!"));
	}

	insertion_sort(&mut v);

	for i in 0..v.len() {
		print!("{}", v[i]);
		if i < v.len() - 1 {
			print!(" ");
		}
	}
}

fn insertion_sort(v: &mut Vec<i32>) {
	for i in 1..v.len() {
		for j in (1..i+1).rev() {
			if v[j] < v[j - 1] {
				let temp = v[j] + v[j - 1];
				v[j] = v[j - 1];
				v[j - 1] = temp - v[j];
			}
		}
	}
}