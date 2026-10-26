use std::{collections::HashMap, time::Duration};

use axum::{
    Json,
    extract::{Query, State},
};
use axum_anyhow::ApiResult;
use sea_orm::{ColumnTrait, EntityTrait, ExprTrait, QueryFilter, QuerySelect, sqlx::types::chrono};

use crate::{
    AppState,
    entities::{scores, users},
    users::OptionalUserInfo,
};
#[derive(Debug, serde::Serialize)]
pub struct ScoreboardEntry {
    username: String,
    sum: i32,
    you: bool,
    lasth: i32,
}

pub async fn scoreboard(
    State(state): State<AppState>,
    Query(userinfo): Query<OptionalUserInfo>,
) -> ApiResult<Json<Vec<ScoreboardEntry>>> {
    let internal = match get_scoreboard(&state).await {
        Ok(d) => d,
        Err(err) => {

            println!("ERR: {err:#?}");
            return Err(err.into());
        },
    };

    return Ok(Json(
        internal
            .into_iter()
            .map(|u| ScoreboardEntry {
                username: u.username,
                sum: u.sum,
                you: userinfo
                    .user_id
                    .as_ref()
                    .map(|id| *id == u.user_id)
                    .unwrap_or(false),
                lasth: u.lasth,
            })
            .collect(),
    ));
}

//Perf really does not matter, so we do it the easy way.

struct InternalScoreboardInfo {
    user_id: String,
    username: String,
    sum: i32,
    lasth: i32,
}

async fn get_scoreboard(state: &AppState) -> anyhow::Result<Vec<InternalScoreboardInfo>> {
    let users = users::Entity::find().all(&state.db).await?;

    let total_scores: HashMap<_, _> = scores::Entity::find()
        .select_only()
        .column(scores::Column::UserId)
        .column_as(scores::Column::Value.sum().cast_as("integer"), "sum")
        .group_by(scores::Column::UserId)
        .into_tuple::<(String, Option<i32>)>()
        .all(&state.db)
        .await?
        .into_iter()
        .collect();

    let last_h: HashMap<_, _> = scores::Entity::find()
        .select_only()
        .column(scores::Column::UserId)
        .column_as(scores::Column::Value.sum().cast_as("integer"), "sum")
        .filter(scores::Column::Submittet.gte(chrono::Local::now() - Duration::from_hours(1)))
        .group_by(scores::Column::UserId)
        .into_tuple::<(String, Option<i32>)>()
        .all(&state.db)
        .await?
        .into_iter()
        .collect();

    return Ok(users
        .into_iter()
        .map(|u| InternalScoreboardInfo {
            sum: total_scores
                .get(&u.user_id)
                .copied()
                .unwrap_or_default()
                .unwrap_or_default(),
            lasth: last_h
                .get(&u.user_id)
                .copied()
                .unwrap_or_default()
                .unwrap_or_default(),
            user_id: u.user_id,
            username: u.username.unwrap(),
        })
        .collect());
}
