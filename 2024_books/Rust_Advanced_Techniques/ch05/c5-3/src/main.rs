#![allow(dead_code)]

fn main() {
	// println!("Hello, world!");

	let mut bicycle_builder = BicycleBuilder::new();

	bicycle_builder
		.with_make("Huffy")
		.with_model("Radio")
		.with_size(46)
		.with_color("red");

	let bicycle = bicycle_builder.build();
	println!("==> My new bike: {:#?}", bicycle);

	let bicycle: Bicycle = Bicycle::new()
		.with_make("Huffy")
		.with_model("Radio")
		.with_size(46)
		.with_color("red");

	println!("==> My new bike: {:#?}", bicycle);
}

#[derive(Debug, Default)]
struct Bicycle {
	make: String,
	model: String,
	size: i32,
	color: String,
}

impl Bicycle {
	fn make(&self) -> &String {
		&self.make
	}
	fn model(&self) -> &String {
		&self.model
	}
	fn size(&self) -> i32 {
		self.size
	}
	fn color(&self) -> &String {
		&self.color
	}

	fn new() -> Self {
		Bicycle { make: String::new(), model: String::new(), size: 0, color: String::new() }
	}

	fn with_make(mut self, make: &str) -> Self {
		self.make = make.into();
		self
	}

	fn with_model(mut self, model: &str) -> Self {
		self.model = model.into();
		self
	}

	fn with_size(mut self, size: i32) -> Self {
		self.size = size;
		self
	}

	fn with_color(mut self, color: &str) -> Self {
		self.color = color.into();
		self
	}
}

struct BicycleBuilder {
	bicycle: Bicycle,
}

impl BicycleBuilder {
	fn new() -> Self {
		Self {
			bicycle: Bicycle {
				make: String::new(),
				model: String::new(),
				size: 0,
				color: String::new(),
			},
		}
	}

	fn with_make(&mut self, make: &str) -> &mut Self {
		self.bicycle.make = make.into();
		self
	}

	fn with_model(&mut self, model: &str) -> &mut Self {
		self.bicycle.model = model.into();
		self
	}

	fn with_size(&mut self, size: i32) -> &mut Self {
		self.bicycle.size = size;
		self
	}

	fn with_color(&mut self, color: &str) -> &mut Self {
		self.bicycle.color = color.into();
		self
	}

	fn build(self) -> Bicycle {
		self.bicycle
	}
}
