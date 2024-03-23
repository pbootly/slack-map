use std::error::Error;
use std::net::TcpListener;
use slackmap::startup::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let address = format!("127.0.0.1:{}", 8081);
    let listener = TcpListener::bind(address)?;
    let _ = run(listener)?.await;
    
    Ok(())
}
