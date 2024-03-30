use std::arch::asm;

fn main() {
	let message = "Hello world from raw syscall!\n";
	let message = String::from(message);

	syscall(message);
}

#[inline(never)]
fn syscall(message: String) {
	let msg_ptr = message.as_ptr();
	let len = message.len();

	unsafe {
		asm!(
			"mov rax, 1", // make a write
			"mov rdi, 1", // write to stdout
			"syscall", // This instruction issues a software interrupt, and the CPU passes on control to the OS.
			in("rsi") msg_ptr,
			in("rdx") len,
			out("rax") _,
			out("rdi") _,
			lateout("rsi") _,
			lateout("rdx") _
		);
	}
}
