// code for linking application info on win taken directly from checklister
fn main() {
	#[cfg(target_os = "windows")]
	println!("cargo:rustc-link-arg=resources.res");
}