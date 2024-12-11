// @generated automatically by Diesel CLI.

diesel::table! {
    meta (id) {
        id -> Integer,
        descricao -> Text,
        data_inicio -> Text,
        data_fim -> Nullable<Text>,
        data_fim_user -> Nullable<Text>,
    }
}

diesel::table! {
    tarefa (id) {
        id -> Integer,
        descricao -> Text,
        data_inicio -> Text,
        data_fim -> Nullable<Text>,
        data_fim_user -> Nullable<Text>,
        id_meta -> Nullable<Integer>,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        email -> Text,
        senha -> Text,
    }
}

diesel::joinable!(tarefa -> meta (id_meta));

diesel::allow_tables_to_appear_in_same_query!(
    meta,
    tarefa,
    users,
);
