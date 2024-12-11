#![feature(trivial_bounds)]
pub mod models;
pub mod schema;

use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use std::env;
use dotenvy::dotenv;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

use crate::models::{User, NewUser, Meta, NewMeta, Tarefa, NewTarefa};

pub fn create_user(conn: &mut SqliteConnection, username: &str, email: &str, senha: &str) -> User {
    use crate::schema::users::dsl::users; // Certifique-se de importar corretamente

    let new_user = NewUser {
        username: username.to_string(),  
        email: email.to_string(),        
        senha: senha.to_string(),        
    };

    diesel::insert_into(users)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user")
}
 
pub fn create_meta(conn: &mut SqliteConnection, descricao: &str, data_inicio: &str, data_fim: Option<&str>, data_fim_user: Option<&str>) -> Meta {
    use crate::schema::meta::dsl::meta; // Certifique-se de importar corretamente

    let new_meta = NewMeta {
        descricao: descricao.to_string(),  
        data_inicio: data_inicio.to_string(),        
        data_fim: Some(data_fim.map(|x| x.to_string()).unwrap_or_default()),
        data_fim_user: Some(data_fim_user.map(|x| x.to_string()).unwrap_or_default()),       
    };

    diesel::insert_into(meta)
        .values(&new_meta)
        .get_result(conn)
        .expect("Error saving new meta")
}

pub fn create_tarefa(conn: &mut SqliteConnection, descricao: &str, data_inicio: &str, data_fim: Option<&str>, data_fim_user: Option<&str>, id_meta: Option<i32>) -> Tarefa {
    use crate::schema::tarefa::dsl::tarefa; // Certifique-se de importar corretamente

    let new_tarefa = NewTarefa {
        descricao: descricao.to_string(),  
        data_inicio: data_inicio.to_string(),        
        data_fim: Some(data_fim.map(|x| x.to_string()).unwrap_or_default()),
        data_fim_user: Some(data_fim_user.map(|x| x.to_string()).unwrap_or_default()),
        id_meta: id_meta.expect("REASON"),       
    };

    diesel::insert_into(tarefa)
        .values(&new_tarefa)
        .get_result(conn)
        .expect("Error saving new tarefa")
}

