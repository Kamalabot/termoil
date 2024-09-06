use std::error::Error;
use std::io::stdin;
use termoil::api::get_response;

// only if it is tokio::main, then it can be async
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut your_input = String::new();
    println!("Provide your query here: ");
    stdin().read_line(&mut your_input).unwrap();
    let response = get_response(your_input, 300).await?;
    println!("Response: {:?}", response);
    Ok(())
}
