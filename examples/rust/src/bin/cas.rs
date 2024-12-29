use scodok::Session;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
  let session = Session::new("https://notes9.iutlan.univ-rennes1.fr");
  let url = scodok::retrieve_cas_url(&session).await?;
  println!("{url}");

  Ok(())
}
