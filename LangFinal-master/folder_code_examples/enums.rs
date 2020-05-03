enum Dog {

	Fluffy,
	Wolf,
	Fat(i64),
	Small(i32)
}

fn greet(dog: Dog) {

	match dog {
		Dog::Fluffy => println!("Fluffy Dog"),
		Dog::Wolf => println!("Run"),
		Dog::Fat(w) => println!("Wow that's a big dog {} ", w),
		Dog::Small(w) => println!("Wow that's a small dog {}", w),
	}
}

fn main()  {
	let fluffy_dog = Dog::Fluffy;
	let wolf_dog   = Dog::Wolf;
	let fat_dog    = Dog::Fat(200);
	let small_dog  = Dog::Small(7);

	greet(fluffy_dog);
	greet(wolf_dog);
	greet(fat_dog);
	greet(small_dog);

}