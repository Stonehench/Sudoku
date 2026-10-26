use axum::{Json, extract::State};
use axum_anyhow::ApiResult;
use migration::prelude::chrono::{self};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};

use crate::{AppState, entities::scores, users::UserInfo};

#[derive(Debug, serde::Serialize)]
pub struct StreakInfo {
    streak: u32,
    multiplier: f32,
}

pub async fn streak(
    State(state): State<AppState>,
    Json(userinfo): Json<UserInfo>,
) -> ApiResult<Json<StreakInfo>> {
    let scores = match scores::Entity::find()
        .filter(scores::Column::UserId.eq(&userinfo.user_id))
        .order_by(scores::Column::Submittet, sea_orm::Order::Desc)
        .all(&state.db)
        .await
    {
        Ok(s) => s,
        Err(err) => {
            println!("ERR: {err}");
            return Err(err.into());
        }
    };

    let Some(latest) = scores.first() else {
        // Has never done a sudoku. No streak.
        return Ok(Json(StreakInfo {
            streak: 0,
            multiplier: 1.0,
        }));
    };

    let now = chrono::Local::now();

    let today = now.date_naive();
    let yesterday = now
        .checked_sub_days(chrono::Days::new(1))
        .unwrap()
        .date_naive();

    let submittet = latest.submittet.unwrap().date_naive();

    if submittet != today && submittet != yesterday {
        return Ok(Json(StreakInfo {
            streak: 0,
            multiplier: 1.0,
        }));
    }

    let mut streak = 1;
    let mut lastdate = submittet;

    for sudoku in scores.iter().skip(1) {
        match (lastdate - sudoku.submittet.unwrap().date_naive()).num_days() {
            0 => continue,
            1 => {
                streak += 1;
                lastdate = sudoku.submittet.unwrap().date_naive()
            }
            _ => break,
        }
    }

    Ok(Json(StreakInfo {
        streak,
        multiplier: 1.1f32.powi(streak as i32),
    }))
}
