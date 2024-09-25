#![allow(dead_code)]

fn main() {
	// 1.
	let bits = BitCount(8);
	let bytes = ByteCount(12);
	dbg!(&bits);
	dbg!(&bytes);

	// type convertions: as_, to_, into, borrow, borrow_mut

	dbg!(bits.to_bytes());
	dbg!(bytes.to_bits());

	dbg!(bits.0);
	dbg!(bytes.0);

	// 2.
	let bits: BitCount = bytes.into();
	dbg!(&bits);

	// 3.
	let xbit = XBit::new(32);
	xbit.as_ref().hello();

	let bits: BitCount = xbit.into();
	bits.hello();
}

#[derive(Debug)]
struct BitCount(u32);

#[derive(Debug)]
struct ByteCount(u32);

impl BitCount {
	fn to_bytes(&self) -> ByteCount {
		ByteCount(self.0 / 8)
	}

	pub fn hello(&self) {
		println!("~~ Hello, world!");
	}
}

impl ByteCount {
	fn to_bits(&self) -> BitCount {
		BitCount(self.0 * 8)
	}
}

impl From<ByteCount> for BitCount {
	fn from(bytes: ByteCount) -> BitCount {
		BitCount(bytes.0 * 8)
	}
}

struct XBit {
	item: BitCount,
}

impl XBit {
	fn new(count: u32) -> Self {
		Self { item: BitCount(count) }
	}
}

impl AsRef<BitCount> for XBit {
	fn as_ref(&self) -> &BitCount {
		&self.item
	}
}

impl Into<BitCount> for XBit {
	fn into(self) -> BitCount {
		self.item
	}
}
