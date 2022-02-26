#[derive(Debug, clap::Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Arguments {
	#[clap(short, long)]
	pub configuration_file: String,
}
