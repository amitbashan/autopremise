use clap::Parser;

pub mod configuration;
pub mod command_line;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
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

    log4rs::init_file("src/configuration/log4rs.yaml", Default::default()).unwrap();
    futures::future::join_all(threads).await;

    Ok(())
}
