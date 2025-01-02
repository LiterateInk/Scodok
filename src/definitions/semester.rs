use serde::Deserialize;

#[derive(Deserialize)]
pub struct Semester {
  pub titre: String,         // "BUT MMI"
  pub formsemestre_id: u16,
  pub semestre_id: u16,
  pub annee_scolaire: String // "2024/2025"
}
