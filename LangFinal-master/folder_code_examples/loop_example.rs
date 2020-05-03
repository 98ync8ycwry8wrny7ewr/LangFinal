fn main() {
	
	'outer_loop: loop {
		println!("Hello and welcome to my outer loop");

		'inner_loop: loop {
			println!("Hello and welcome to my inner loop");

			break 'outer_loop
		}
	}
}