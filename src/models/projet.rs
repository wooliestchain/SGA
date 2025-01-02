use serde::{Deserialize, Serialize};
use super::utilisateur;

#[derive(Debug, Serialize, Deserialize)]
pub struct Projet{
    pub id: i32,
    pub nom: String,
    pub description: String,
    pub ministere_responsable: String,
    pub referant_projet: String,
    pub referant_presidence: String,
    pub ville: String,
    pub categorie: String,
    pub annee_debut: i32,
    pub annee_fin: i32,
    pub type_projet: String,
    pub personnelles_alloues: String,
    pub infrastructures_mobilises: String,
    pub fonds_decaisses: f64,
    pub source_financement: String,
    pub date_creation: String,
    pub statut: String,
    pub priorite: String,
    pub risques_associes: String,
    pub objectifs: String,
    pub impact_attendu: String,
    pub commentaires: String,
}

impl Projet {
    fn get_nom (&self) -> &str{
        &self.nom
    }
}