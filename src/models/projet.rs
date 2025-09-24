use serde::{Deserialize, Serialize};
use super::utilisateur::UtilisateurProjetInput;

#[derive(Debug, Serialize, Deserialize)]
pub struct Projet{
    pub id: Option<i32>,
    pub nom: String,
    pub code: i32,
    pub description: String,
    pub annee_debut: i32,
    pub annee_fin: i32,
    pub date_creation: Option<String>,
    pub impact_attendu: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjetInput {
    pub nom: String,
    pub code: i32,
    pub description: Option<String>,
    pub annee_debut: Option<i32>,
    pub annee_fin: Option<i32>,
    pub impact_attendu: Option<String>,
    pub statut: Option<String>,
    pub type_projet: Option<String>,
    pub priorite: Option<String>,
    pub objectifs: Option<String>,
    pub utilisateurs: Option<Vec<UtilisateurProjetInput>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Categorie{
    pub id: Option<i32>,
    pub nom: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ProjetCategorie{
    pub projet_id: i32,
    pub categorie_id: i32,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ville{
    pub id: Option<i32>,
    pub nom: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfrastructureMobilisee{
    pub id: Option<i32>,
    pub nom: String,
    pub date_mise_a_jour: Option<String>,
}
/* 

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjetVille{
    pub projet_id: i32,
    pub ville_id: i32,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct ProjetInfrastructure{
    pub projet_id: i32,
    pub infrastructure_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FondsDecaisse{
    pub id: Option<i32>,
    pub projet_id: i32,
    pub montant: f64,
    pub date_mise_a_jour: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Risque{
    pub id: Option<i32>,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjetRisque{
    pub projet_id: i32,
    pub risque_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commentaire{
    pub id: Option<i32>,
    pub projet_id: i32,
    pub contenu: String,
    pub date_creation: String,
}

*/