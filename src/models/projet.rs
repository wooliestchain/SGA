use serde::{Deserialize, Serialize};
use super::utilisateur;

#[derive(Debug, Serialize, Deserialize)]
pub struct Projet{
    pub id: Option<i32>,
    pub nom: String,
    pub code: i32,
    pub description: String,
    pub ministere_responsable: String,
    pub referant_projet: String,
    pub referant_presidence: String,
    pub annee_debut: i32,
    pub annee_fin: i32,
    pub type_projet: String,
    pub source_financement: String,
    pub date_creation: Option<String>,
    pub statut: String,
    pub priorite: String,
    pub objectifs: String,
    pub impact_attendu: String,
}

pub struct Categorie{
    pub id: Option<i32>,
    pub nom: String,
}

pub struct ProjetCategorie{
    pub projet_id: i32,
    pub categorie_id: i32,
}

pub struct Ville{
    pub id: Option<i32>,
    pub nom: String,
}

pub struct ProjetVille{
    pub projet_id: i32,
    pub ville_id: i32,
}

pub struct InfrastructureMobilisee{
    pub id: Option<i32>,
    pub nom: String,
    pub date_mise_a_jour: String,
}

pub struct ProjetInfrastructure{
    pub projet_id: i32,
    pub infrastructure_id: i32,
}

pub struct FondsDecaisse{
    pub id: Option<i32>,
    pub projet_id: i32,
    pub montant: f64,
    pub date_mise_a_jour: String,
}

pub struct Risque{
    pub id: Option<i32>,
    pub description: String,
}

pub struct ProjetRisque{
    pub projet_id: i32,
    pub risque_id: i32,
}

pub struct Commentaire{
    pub id: Option<i32>,
    pub projet_id: i32,
    pub contenu: String,
    pub date_creation: String,
}