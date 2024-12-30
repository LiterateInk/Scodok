use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
  let instance_url = "https://notes.scodoc.local/";
  let cas_url = scodok::retrieve_cas_url(instance_url).await?;
  println!("You have to authenticate to your CAS at the following URL: {cas_url}");

  // Let's say we grabbed this variable from the CAS authentication...
  let ticket = "ST-123456789-SOMELONGSTRING-vmjava-pcas2";
  let session = scodok::process_cas_ticket(instance_url, ticket).await?;
  println!("You are now authenticated in ScoDoc Notes with the following PHP session ID: {}", session.php_sess_id());

  Ok(())
}
