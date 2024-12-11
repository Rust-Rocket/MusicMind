#[macro_use]
extern crate rocket;

//use rocket::http::Status;
use diesel::prelude::*;
use rocket_app_rotas::models::{User, NewUser, UpdateUser, Meta, NewMeta, UpdateMeta};
use rocket_app_rotas::schema::users::dsl::users;
use rocket_app_rotas::schema::meta::dsl::meta;
//use rocket_app_rotas::schema::tarefa::dsl::tarefa;
use rocket_app_rotas::establish_connection;
use rocket::serde::json::{Json, Value as JsonValue};
use serde_json::json; 


#[get("/meta")]
fn list_meta() -> Json<JsonValue> {
    let connection = &mut establish_connection();

    let results = meta
        .load::<Meta>(connection)
        .expect("Erro ao consultar users");

    Json(JsonValue::from(json!({
        "meta": results,
    })))
}

#[get("/users")]
fn list_users() -> Json<JsonValue> {
    let connection = &mut establish_connection();

    let results = users
        .load::<User>(connection)
        .expect("Erro ao consultar users");

    Json(JsonValue::from(json!({
        "users": results,
    })))
}

#[post("/meta", format = "json",data = "<new_meta>")]
fn create_meta(new_meta: Json<NewMeta>) -> Json<JsonValue> {

    let new_meta = NewMeta {
        descricao: new_meta.descricao.clone(),
        data_inicio: new_meta.data_inicio.clone(),
        data_fim: new_meta.data_fim.clone(),
        data_fim_user: new_meta.data_fim_user.clone(),
    };

    let connection = &mut establish_connection();

    diesel::insert_into(meta)
        .values(&new_meta)
        .execute(connection)
        .expect("erro ao criar user");

    Json(JsonValue::from(json!({
        "status": "success",
        "message": "Meta has been created",    
    })))
}

// Rota POST para criar um novo usuário
#[post("/users", format = "json",data = "<new_user>")]
fn create_user(new_user: Json<NewUser>) -> Json<JsonValue> {

    let new_user = NewUser {
        username: new_user.username.clone(),
        email: new_user.email.clone(),
        senha: new_user.senha.clone(),
    };

    let connection = &mut establish_connection();

    diesel::insert_into(users)
        .values(&new_user)
        .execute(connection)
        .expect("erro ao criar user");

    Json(JsonValue::from(json!({
        "status": "success",
        "message": "Student has been created",    
    })))
}

#[put("/meta/<id>", data = "<updated_meta>")]
fn edit_meta(id: i32, updated_meta: Json<UpdateMeta>) -> Json<JsonValue> {

    let updated_meta = UpdateMeta {
        descricao: updated_meta.descricao.clone(),
        data_inicio: updated_meta.data_inicio.clone(),
        data_fim: updated_meta.data_fim.clone(),
        data_fim_user: updated_meta.data_fim_user.clone(),
    };

    let connection = &mut establish_connection();

    diesel::update(meta.find(id))
        .set(&updated_meta)
        .execute(connection)
        .expect("erro ao editar user");

    Json(JsonValue::from(json!({
        "status": "success",
        "message": "Meta has been edited",    
    })))
}

// rota para editar user (alterado para POST para compatibilidade)
#[put("/users/<id>", data = "<updated_user>")]
fn edit_user(id: i32, updated_user: Json<UpdateUser>) -> Json<JsonValue> {

    // Cria uma estrutura de atualização com os novos dados
    let updated_user = UpdateUser {
        username: updated_user.username.clone(),
        email: updated_user.email.clone(),
        senha: updated_user.senha.clone(),
    };

    let connection = &mut establish_connection();

    diesel::update(users.find(id))
        .set(&updated_user)
        .execute(connection)
        .expect("erro ao editar user");

    Json(JsonValue::from(json!({
        "status": "success",
        "message": "Student has been edited",    
    })))
}

#[delete("/meta/<id>")]
fn delete_meta(id: i32) -> Json<JsonValue> {
    let connection = &mut establish_connection();

    diesel::delete(meta.find(id))
        .execute(connection)
        .expect("erro ao deletar user");

    Json(JsonValue::from(json!({
        "status": "success",
        "message": "Meta has been deleted",    
    })))
}

//rota para deletar user
#[delete("/users/<id>")]
fn delete_user(id: i32) -> Json<JsonValue> {
    let connection = &mut establish_connection();

    // Deleta o usuário
    diesel::delete(users.find(id))
        .execute(connection)
        .expect("erro ao deletar user");

    Json(JsonValue::from(json!({
        "status": "success",
        "message": "Student has been deleted",    
    })))
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![ 
            list_users,  
            create_user,  
            edit_user, 
            delete_user,
            list_meta,  
            create_meta,  
            edit_meta, 
            delete_meta,
        ])
}
