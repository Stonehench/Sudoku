use std::str::FromStr;

use axum::{Form, extract::State};
use axum_anyhow::ApiResult;
use migration::prelude::chrono;
use sea_orm::{ActiveValue, EntityTrait};

use crate::{AppState, entities::scores};

#[derive(Debug, serde::Deserialize)]
pub struct AddScore {
    user_id: String,
    value: i32,
    daily_dato: Option<String>,
}

pub async fn add_score(State(state): State<AppState>, Form(form): Form<AddScore>) -> ApiResult<()> {
    let daily_dato = match form.daily_dato {
        Some(d) => ActiveValue::Set(Some(chrono::NaiveDate::from_str(&d)?)),
        None => ActiveValue::Set(None),
    };

    scores::Entity::insert(scores::ActiveModel {
        score_id: ActiveValue::NotSet,
        user_id: ActiveValue::Set(form.user_id),
        value: ActiveValue::Set(form.value),
        submittet: ActiveValue::Set(chrono::Local::now().into()),
        daily_dato,
    })
    .exec(&state.db)
    .await?;

    Ok(())
}
