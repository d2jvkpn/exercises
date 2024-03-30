use std::io;

fn main() {
	let message = "Hello world from normal syscall!\n";
	let message = String::from(message);

	syscall(message).unwrap();
}

#[cfg(target_family = "unix")]
#[link(name = "c")]
extern "C" {
	fn write(fd: u32, buf: *const u8, count: usize) -> i32;
}

#[cfg(target_family = "unix")]
fn syscall(message: String) -> io::Result<()> {
	let msg_ptr = message.as_ptr();
	let len = message.len();
	let res = unsafe { write(1, msg_ptr, len) };

	if res == -1 {
		return Err(io::Error::last_os_error());
	}

	Ok(())
}

/*
#[link(name = "kernel32")]
extern "system" {
	fn GetStdHandle(nStdHandle: i32) -> i32;
	fn WriteConsoleW(
		hConsoleOutput: i32,
		lpBuffer: *const u16,
		numberOfCharsToWrite: u32,
		lpNumberOfCharsWritten: *mut u32,
		lpReserved: *const std::ffi::c_void,
	) -> i32;
}
*/
