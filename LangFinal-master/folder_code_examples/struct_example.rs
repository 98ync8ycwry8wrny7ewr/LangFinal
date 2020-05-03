fn main() {
	struct Coords1 {
		lat: f64,
		lng: f64,
	}

	struct Coords2(f32, f32);

	let test1 = Coords1{lat: 1.0, lng: 2.0};
	let test2 = Coords2(1.0, 2.0);
}