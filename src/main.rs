use rusty_project::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}
