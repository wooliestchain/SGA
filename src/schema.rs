diesel::table! {
    projet (id) {
        id -> Int4,
        nom -> Varchar,
        code -> Int4,
        description -> Nullable<Text>,
        annee_debut -> Nullable<Int4>,
        annee_fin -> Nullable<Int4>,
        date_creation -> Nullable<Timestamp>,
        impact_attendu -> Nullable<Text>,
    }
}

diesel::table! {
    statut (id) {
        id -> Int4,
        projet_id -> Int4,
        statut_actuel -> Varchar,
        date_mise_a_jour -> Nullable<Timestamp>,
    }
}

diesel::table! {
    type (id) {
        id -> Int4,
        projet_id -> Int4,
        type -> Varchar,
        date_mise_a_jour -> Nullable<Timestamp>,
    }
}

diesel::table! {
    objectifs (id) {
        id -> Int4,
        projet_id -> Int4,
        description -> Text,
        date_creation -> Nullable<Timestamp>,
    }
}

diesel::table! {
    priorite (id) {
        id -> Int4,
        projet_id -> Int4,
        description -> Text,
        date_creation -> Nullable<Timestamp>,
    }
}

diesel::table! {
    ministere (id) {
        id -> Int4,
        projet_id -> Int4,
        nom -> Varchar,
        code -> Int4,
    }
}

diesel::table! {
    sourcefinancement (id) {
        id -> Int4,
        projet_id -> Int4,
        nom -> Nullable<Varchar>,
        code -> Int4,
    }
}

diesel::table! {
    user (id) {
        id -> Int4,
        nom -> Varchar,
        email -> Varchar,
        mot_de_passe -> Varchar,
        role -> Varchar,
    }
}

diesel::table! {
    projet_user (id) {
        id -> Int4,
        projet_id -> Int4,
        user_id -> Int4,
        role -> Varchar,
    }
}

diesel::table! {
    categorie (id) {
        id -> Int4,
        projet_id -> Int4,
        nom -> Varchar,
    }
}

diesel::table! {
    ville (id) {
        id -> Int4,
        projet_id -> Int4,
        nom -> Varchar,
    }
}

diesel::table! {
    infrastructuremobilisee (id) {
        id -> Int4,
        projet_id -> Int4,
        nom -> Varchar,
        date_mise_a_jour -> Nullable<Timestamp>,
    }
}

diesel::table! {
    fondsdecaisse (id) {
        id -> Int4,
        projet_id -> Int4,
        montant -> Numeric,
        date_mise_a_jour -> Nullable<Timestamp>,
    }
}

diesel::table! {
    risque (id) {
        id -> Int4,
        projet_id -> Int4,
        description -> Text,
    }
}

diesel::table! {
    commentaire (id) {
        id -> Int4,
        projet_id -> Int4,
        contenu -> Text,
        date_creation -> Nullable<Timestamp>,
    }
}

diesel::joinable!(statut -> projet (projet_id));
diesel::joinable!(type -> projet (projet_id));
diesel::joinable!(objectifs -> projet (projet_id));
diesel::joinable!(priorite -> projet (projet_id));
diesel::joinable!(ministere -> projet (projet_id));
diesel::joinable!(sourcefinancement -> projet (projet_id));
diesel::joinable!(projet_user -> projet (projet_id));
diesel::joinable!(projet_user -> user (user_id));
diesel::joinable!(categorie -> projet (projet_id));
diesel::joinable!(ville -> projet (projet_id));
diesel::joinable!(infrastructuremobilisee -> projet (projet_id));
diesel::joinable!(fondsdecaisse -> projet (projet_id));
diesel::joinable!(risque -> projet (projet_id));
diesel::joinable!(commentaire -> projet (projet_id));

diesel::allow_tables_to_appear_in_same_query!(
    projet,
    statut,
    type,
    objectifs,
    priorite,
    ministere,
    sourcefinancement,
    user,
    projet_user,
    categorie,
    ville,
    infrastructuremobilisee,
    fondsdecaisse,
    risque,
    commentaire,
);
