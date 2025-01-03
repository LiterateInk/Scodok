use super::UserStatus;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
  pub passerelle_version: String, // "6:3:4"
  pub histogramme: bool,
  pub message_non_publication_releve: String,
  #[serde(rename = "releve_PDF")]
  pub releve_pdf: bool,
  #[serde(rename = "liste_dep_publi_PDF")]
  pub liste_dep_publi_pdf: String,
  pub etudiant_modif_photo: bool,
  pub acces_enseignants: bool,
  pub cloisonner_enseignants: bool,
  pub analystics_interne: bool,
  pub envoi_donnees_version: bool,
  pub analyse_temps_requetes: bool,
  #[serde(rename = "nom_IUT")]
  pub nom_iut: String, // "IUT de Lannion"
  pub doc_afficher_nip: bool,
  pub doc_afficher_id: bool,
  pub doc_afficher_date_naissance: bool,
  #[serde(rename = "idReg")]
  pub id_reg: String,
  #[serde(rename = "idPlaceHolder")]
  pub id_placeholder: String, // "Identifiant CAS"
  #[serde(rename = "idInfo")]
  pub id_info: String, // "Ajoutez l'identifiant CAS"
  #[serde(rename = "nameReg")]
  pub name_reg: String, // "^.+$"
  #[serde(rename = "namePlaceHolder")]
  pub name_placeholder: String, // "Nom utilisateur"
  #[serde(rename = "nameInfo")]
  pub name_info: String, // "Indiquez le nom"
  pub module_absences: bool,
  pub afficher_absences: bool,
  pub data_absences_scodoc: bool,
  pub metrique_absences: String, // "heure"
  pub autoriser_justificatifs: bool,
  pub liste_dep_ok_justificatifs: String,
  /// List separated by commas.
  pub liste_dep_publi_absences: String, // "Infocom,Informatique,MMI,MP,RT,TN"
  pub message_rapport_absences: String,
  pub message_justificatifs: String,
  #[serde(rename = "absence_heureDebut")]
  pub absence_heure_debut: f32, // 8
  #[serde(rename = "absence_heureFin")]
  pub absence_heure_fin: f32, // 20
  pub absence_pas: f32, // 0.5
  #[serde(rename = "absence_dureeSeance")]
  pub absence_duree_seance: f32, // 2
  pub session: String,
  pub name: String, // "Doe John"
  pub statut: UserStatus,
}
