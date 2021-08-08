static mut COUNTER: i32 = 0;

#[no_mangle]
pub unsafe fn inc(i: i32) -> i32 { 
	COUNTER += i;

	COUNTER
}

#[no_mangle]
pub unsafe fn dump(msg: &[u8]) {

	let mut result: String = String::from("");
	for i in 0..99 {
		let c = char::from(msg[i]);
		if c == '\0' { break };
		result.push(c);
	}
	println!("{}: {}", result, COUNTER);
}
