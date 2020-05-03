fn main() {
	let mut count = 0;

	let res = loop {
		count += 1;

		if count == 10 {
			break count;
		}
	};

	println!("{:?}", res );
}