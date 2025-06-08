use mycron::run_app;

#[tokio::main]
async fn main() {
    if let Err(e) = run_app().await {
        e.exit()
    }
}
