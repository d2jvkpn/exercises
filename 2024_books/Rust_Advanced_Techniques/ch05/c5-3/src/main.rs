#![allow(dead_code)]

macro_rules! with_str {
	($func:ident, $name:ident) => {
		fn $func(mut self, $name: &str) -> Self {
			self.$name = $name.into();
			self
		}
	};
}

macro_rules! with_type {
	($func:ident, $name:ident, $type:ty) => {
		fn $func(mut self, $name: $type) -> Self {
			//self.$name = $name;
			//self
			Self { $name: $name.into(), ..self }
		}
	};
}

macro_rules! accessor {
	($func:ident, &$ret:ty) => {
		pub fn $func(&self) -> &$ret {
			&self.$func
		}
	};

	($func:ident, $ret:ty) => {
		pub fn $func(&self) -> $ret {
			self.$func
		}
	};
}

fn main() {
	// 1.
	let mut bicycle_builder = BicycleBuilder::new();

	bicycle_builder
		.with_make("Huffy")
		.with_model("Radio")
		.with_size(46)
		.with_color("red");

	let bicycle = bicycle_builder.build();
	println!("==> My new bike: {:#?}", bicycle);

	// 2.
	let bicycle: Bicycle = Bicycle::new()
		.with_make("Huffy")
		.with_model("Radio")
		.with_size(46)
		.with_color("red");

	println!("==> My new bike: {:#?}", bicycle);

	println!("==> color={:?}, size={}", bicycle.color(), bicycle.size());
}

#[derive(Debug, Default)]
struct Bicycle {
	make: String,
	model: String,
	size: i32,
	color: String,
}

impl Bicycle {
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

	/*
	fn with_size(mut self, size: i32) -> Self {
		self.size = size;
		self
	}
	*/
	with_type!(with_size, size, i32);

	/*
	fn with_color(mut self, color: &str) -> Self {
		self.color = color.into();
		self
	}
	*/
	with_str!(with_color, color);

	fn make(&self) -> &String {
		&self.make
	}

	fn model(&self) -> &String {
		&self.model
	}

	/*
	fn size(&self) -> i32 {
		self.size
	}
	*/
	accessor!(size, i32);

	/*
	fn color(&self) -> &String {
		&self.color
	}
	*/
	accessor!(color, &String);
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
