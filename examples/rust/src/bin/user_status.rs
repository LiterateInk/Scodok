use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
  let session = scodok::Session::new("https://notes.scodoc.local/", "YOUR_PHPSESSID");

  let user_email = "someone@univ-example.fr"; // Grab this from any way you want...
  let status = scodok::get_user_status(&session, user_email.into()).await?;
  println!("Status of the user '{user_email}' is {status}");

  Ok(())
}
