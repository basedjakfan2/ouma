fn main() {
	println!("cargo:rustc-link-arg=-static");
	println!("cargo:rustc-link-arg=-nostartfiles");
}
