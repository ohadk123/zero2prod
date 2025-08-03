use std::io::Error;

use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run().await
}
