use std::fs::File;
use std::io::Write;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
  let session = scodok::Session::new("https://notes.scodoc.local/", "YOUR_PHPSESSID");
  let profile_picture_bytes = scodok::get_profile_picture_bytes(&session, None).await?;
  println!("Received {} bytes of picture.", profile_picture_bytes.len());

  // Let's save the picture to a file !
  let mut file = File::create("profile-picture.jpg")?;
  file.write_all(&profile_picture_bytes)?;

  Ok(())
}
