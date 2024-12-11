use diesel::{Insertable, Queryable, Selectable};
use crate::schema::{users, meta, tarefa};
use diesel::AsChangeset;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub senha: String,
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = meta)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Meta {
    pub id: i32,
    pub descricao: String,
    pub data_inicio: String,
    pub data_fim: Option<String>,
    pub data_fim_user: Option<String>,
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = tarefa)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Tarefa {
    pub id: i32,
    pub descricao: String,
    pub data_inicio: String,
    pub data_fim: Option<String>,
    pub data_fim_user: Option<String>,
    pub id_meta: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = meta)]
#[derive(Deserialize, Serialize)]
pub struct NewMeta {
    pub descricao: String,
    pub data_inicio: String,
    pub data_fim: Option<String>,
    pub data_fim_user: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = tarefa)]
#[derive(Deserialize, Serialize)]
pub struct NewTarefa {
    pub descricao: String,
    pub data_inicio: String,
    pub data_fim: Option<String>,
    pub data_fim_user: Option<String>,
    pub id_meta: i32,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
#[derive(Deserialize, Serialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub senha: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = meta)]
#[derive(Deserialize, Serialize)]
pub struct UpdateMeta {
    pub descricao: String,
    pub data_inicio: String,
    pub data_fim: Option<String>,
    pub data_fim_user: Option<String>,
}

#[derive(AsChangeset)]
#[diesel(table_name = tarefa)]
#[derive(Deserialize, Serialize)]
pub struct UpdateTarefa {
    pub descricao: String,
    pub data_inicio: String,
    pub data_fim: Option<String>,
    pub data_fim_user: Option<String>,
    pub id_meta: i32,
}

#[derive(AsChangeset)]
#[diesel(table_name = users)]
#[derive(Deserialize, Serialize)]
pub struct UpdateUser {
    pub username: String,
    pub email: String,
    pub senha: String,
}

