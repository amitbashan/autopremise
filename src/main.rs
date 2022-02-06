use clap::Parser;

pub mod configuration;
pub mod command_line;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    tracing_subscriber::fmt::init();

    let arguments = command_line::Arguments::parse();
    let configuration: configuration::Configuration = {
        use tokio::io::AsyncReadExt;

        let mut file = tokio::fs::File::open(arguments.configuration_file).await?;
        let mut data = String::new();

        file.read_to_string(&mut data).await?;
        serde_json::from_str(&data)?
    };
    let threads = configuration.users
        .into_iter()
        .map(|user| tokio::spawn(async {
            user.automate().await.unwrap()
        }));

    futures::future::join_all(threads).await;

    Ok(())
}
