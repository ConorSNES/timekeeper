// code for linking application info on windows taken directly from checklister
fn main() {
	#[cfg(target_os = "windows")]
	println!("cargo:rustc-link-arg=resources.res");
}