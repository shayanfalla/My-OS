#![feature(lang_items)]
#![feature(lang_items)]
#![feature(const_fn)]
#![feature(const_unique_new)]
#![feature(unique)]
#![no_std]
extern crate rlibc;
extern crate volatile;
extern crate spin;

#[macro_use]
mod vga_buffer;

#[no_mangle]
pub extern fn rust_main() {
	
	vga_buffer::clear_screen();
	//println!("Hello World{}", "!");
	//println!("{}", { println!("inner"); "outer"});
	let x = better(12);
	println!("The number is : {}",x);
	//better(x);

	loop{}
}

fn better(n: u32) -> (u32, u32){
	if n == 1{
	(1,0)
} else {
	println!("better({})", n-1);
	let (f1, f2) = better(n-1);
	(f1+f2,f1)
	}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}
