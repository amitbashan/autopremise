pub mod identity_platform {
	pub const KEY: &str = "AIzaSyD_R91Qkp1HpRWtv6JRZjMKE9GzgAkfwsc";
	pub const ENDPOINT: &str = const_format::formatcp!("https://securetoken.googleapis.com/v1/token?key={}", KEY);
}
