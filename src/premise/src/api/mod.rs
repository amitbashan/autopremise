pub mod form_localization;
pub mod option;
pub mod estimated_duration;
pub mod requirement;
pub mod currency;
pub mod price;
pub mod monitoring;
pub mod campaign;
pub mod service_data;
pub mod update;
pub mod location;
pub mod user;
pub mod google;
pub mod task;
pub mod image;
pub mod time;
pub mod number;

#[macro_export]
macro_rules! make_endpoint {
    ($($path:ident)/+) => {
		{
			use const_format::{formatcp, str_replace};

			formatcp!("https://v2.premise.com/api/mobileProxy/{}", str_replace!(stringify!($($path)/+), " ", ""))
		}
	};
}
