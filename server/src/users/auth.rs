use axum::{Form, Json, extract::State};
use axum_anyhow::ApiResult;
use sea_orm::{ConnectionTrait, EntityTrait, raw_sql};
use tracing::error;

use crate::{AppState, entities::users};

#[derive(Debug, serde::Deserialize)]
pub struct LoginReq {
    username: String,
    password: String,
}

#[derive(Debug, serde::Serialize)]
pub struct LoginResp {
    user_id: String,
}

pub async fn login(
    State(state): State<AppState>,
    Form(login_req): Form<LoginReq>,
) -> ApiResult<Json<LoginResp>> {
    let LoginReq { username, password } = login_req;

    let maybe_user = users::Entity::find()
        .from_raw_sql(
            raw_sql!(
                MySql, 
                r#"select * from users where username = {username}  and password = md5(concat(passsalt,{password}))"#
            ),
        )
        .one(&state.db)
        .await
        .unwrap();

    match maybe_user {
        Some(user) => Ok(Json(LoginResp{
            user_id: user.user_id,
        })),
        None => Err(axum_anyhow::not_found("Not found", "Incorrect username or password")),
    }
}

#[derive(Debug,serde::Serialize)]
pub struct RegisterResp  {
    user_id: String,
    username: String
}

// "insert into users values (?,?,md5(concat(md5(?),?)),md5(?))",
pub async fn register(
    State(state): State<AppState>,
    Form(login_req): Form<LoginReq>,
) -> ApiResult<Json<RegisterResp>> { 

    let LoginReq { username, password } = login_req;

    let user_id = uuid::Uuid::new_v4().to_string();
    let salt_prehash = uuid::Uuid::new_v4().to_string();
    if let Err(err) = state.db.execute_raw(raw_sql!(MySql,"insert into users values ({user_id},{username},md5(concat(md5({salt_prehash}),{password})),md5({salt_prehash}))")).await {
        error!("Failed to insert new user into database: {err}");
        return Err(err.into());
    }

    return Ok(Json(RegisterResp { user_id, username }));
}

#[derive(Debug,serde::Deserialize)]
pub struct ChangePasswdReq {
    user_id: String,
    password: String
}

pub async fn change_password(State(state): State<AppState>, Form(login_req): Form<ChangePasswdReq>) -> ApiResult<()> {

    let ChangePasswdReq { user_id, password } = login_req;

    if let Err(err) = state.db.execute_raw(raw_sql!(MySql,"update users set password = md5(concat(passsalt,{password})) where user_id = {user_id}")).await {
        error!("Failed to insert new user into database: {err}");
        return Err(err.into());
    }

    Ok(())
}