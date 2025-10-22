use tokio;

mod auth;
mod connection;
mod peer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = auth::Client::from(
        auth::NewDomain::new("main")
            .create_account()
            .username("bailey")?
            .password("1234")
            .await?,
    );
    Ok(())
}
