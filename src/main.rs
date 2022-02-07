use clap::Parser;
use futures::StreamExt;

pub mod configuration;
pub mod command_line;

const MAXIMUM_USERS: usize = 5;

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

    futures::stream::iter(configuration.users)
        .map(|user| tokio::spawn(async { user.automate().await }))
        .buffer_unordered(MAXIMUM_USERS)
        .for_each(|result| async {
            match result {
                Ok(Ok(client)) =>
                    tracing::info!("Finished all tasks for user {}.", client.user.data.id),
                Ok(Err(error)) =>
                    tracing::error!(r#"Unexpected error occurred while automating user: {}"#, error.to_string()),
                Err(error) => tracing::error!("{}", error.to_string())
            }
        }).await;

    Ok(())
}
