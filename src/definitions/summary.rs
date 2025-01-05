use serde::Deserialize;

#[derive(Deserialize)]
pub struct Summary {
  pub version: String, // "0"
  pub r#type: String,  // NOTE: "BUT", probably an `enum`
  pub date: String,    // "2025-01-01T02:50:07.364238+01:00"
  pub publie: bool,
  pub etat_inscription: String, // NOTE: "I", probably an `enum`
  pub etudiant: SummaryStudent,
  pub formation: SummaryFormation,
  pub formsemestre_id: u16,
  pub options: SummaryOptions,
}

#[derive(Deserialize)]
pub struct SummaryStudent {
  pub boursier: bool,
  pub civilite_etat_civil: Option<String>,
  pub civilite: String,
  pub code_ine: String,
  pub code_nip: String,
  pub date_naissance: String,
  pub dept_acronym: String,
  pub dept_id: u16,
  pub dept_naissance: String,
  pub email: String,
  pub emailperso: String,
  pub etat_civil: String,
  pub etudid: u32,
  pub lieu_naissance: String,
  pub nationalite: String,
  pub nom: String,
  pub nomprenom: String,
  pub prenom_etat_civil: Option<String>,
  pub prenom: String,
  pub fiche_url: String,
  pub photo_url: String,
  pub id: u32,
  pub domicile: String,
  pub villedomicile: String,
  pub telephone: String,
  pub fax: String,
  pub description: String,
  pub codepostaldomicile: String,
  pub paysdomicile: String,
  pub telephonemobile: String,
  pub typeadresse: String,
}

#[derive(Deserialize)]
pub struct SummaryFormation {
  pub id: u16,
  pub acronyme: String,
  pub titre_officiel: String,
  pub titre: String,
}

#[derive(Deserialize)]
pub struct SummaryOptions {
  pub show_abs: bool,
  pub show_abs_modules: bool,
  pub show_ects: bool,
  pub show_codemodules: bool,
  pub show_matieres: bool,
  pub show_rangs: bool,
  pub show_ue_rangs: bool,
  pub show_mod_rangs: bool,
  pub show_moypromo: bool,
  pub show_minmax: bool,
  pub show_minmax_mod: bool,
  pub show_minmax_eval: bool,
  pub show_coef: bool,
  pub show_ue_cap_details: bool,
  pub show_ue_cap_current: bool,
  pub show_temporary: bool,
  pub temporary_txt: String,
  pub show_uevalid: bool,
  pub show_date_inscr: bool,
  pub block_moyenne_generale: bool,
  pub bgcolor: String,
}
