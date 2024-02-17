#![no_std]
use ukrust_sys;
use alloc;
use alloc::vec::Vec;

// dsadasd
#[no_mangle]
pub extern "C" fn test_foo() -> i32 {
	let mut v = Vec::new();
	v.push(33);
	v.push(42);
	v.pop().unwrap()
}
