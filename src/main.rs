use server::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run().await
}
