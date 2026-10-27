use axum::{Form, Json, extract::State};
use axum_anyhow::ApiResult;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, sqlx::types::chrono,
};
use solver::rules::square_rule::SquareRule;

use crate::{
    AppState,
    entities::{daily_challenges, scores},
};

#[derive(serde::Serialize)]
pub struct DailySudoku {
    dato: String,
    puzzle: String,
    solved: Option<bool>,
}

#[derive(Debug, serde::Deserialize)]
pub struct DailyUserId {
    user_id: Option<String>,
}

pub async fn daily(
    State(state): State<AppState>,
    Form(form): Form<DailyUserId>,
) -> ApiResult<Json<DailySudoku>> {
    let DailyUserId { user_id } = form;

    let today = chrono::Local::now();

    let challenge = daily_challenges::Entity::find()
        .filter(daily_challenges::Column::Dato.eq(today.date_naive()))
        .one(&state.db)
        .await
        .map_err(|e| axum_anyhow::internal_error("Internal error", &format!("{e:#?}")))?;

    let puzzle = match challenge {
        Some(daily_model) => daily_model.puzzle,
        None => {
            let (sudoku, _) = solver::sudoku::Sudoku::generate_with_size(
                9,
                vec![SquareRule::new()],
                None,
                solver::sudoku::Difficulty::Medium,
            )
            .unwrap();
            let puzzle = sudoku.to_format();

            let todays_daily = daily_challenges::ActiveModel {
                dato: Set(today.date_naive()),
                puzzle: Set(puzzle.clone()),
            };
            match todays_daily.insert(&state.db).await {
                Ok(_) => {
                    // Successfull insert, this is now the daily sudoku
                    puzzle
                }
                Err(insert_error) => {
                    // Failed to insert, this means that due to a race condition another request generated
                    // the daily while this request did. We use the other one since it finished first.
                    let challenge = daily_challenges::Entity::find()
                        .filter(daily_challenges::Column::Dato.eq(today.date_naive()))
                        .one(&state.db)
                        .await
                        .map_err(|e| {
                            axum_anyhow::internal_error("Internal error", &format!("{e}"))
                        })?.ok_or_else(|| axum_anyhow::internal_error("Internal error", &format!("Daily failed to insert yet did not already exist: {insert_error}")))?;

                    challenge.puzzle
                }
            }
        }
    };

    // If no user, then None. Else check db.
    let solved = if let Some(user_id) = user_id {
        // Check if already solved
        Some(
            scores::Entity::find()
                .filter(scores::Column::DailyDato.eq(today.date_naive()))
                .filter(scores::Column::UserId.eq(user_id))
                .one(&state.db)
                .await?
                .is_some(),
        )
    } else {
        None
    };

    return Ok(Json(DailySudoku {
        dato: today.date_naive().to_string(),
        puzzle,
        solved,
    }));
}
