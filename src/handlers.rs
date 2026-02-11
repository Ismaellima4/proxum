use crate::{
    AppState,
    persitence::PersistenceError,
    protobuf::Protobuf,
    user::Id,
    user_gen::{CreateUserRequest, UserResponse},
    validate::Validate,
};
use axum::{
    extract::{Path, State},
    http::{StatusCode, header},
    response::IntoResponse,
};

pub async fn find_user_by_id(
    Path(id): Path<Id>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    match state.find_user_by_id(id).await {
        Ok(Some(user)) => Protobuf::<UserResponse>(user.into()).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn delete_by_id(Path(id): Path<Id>, State(state): State<AppState>) -> impl IntoResponse {
    match state.delete_by_id(id).await {
        Ok(_bool) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn save(
    State(state): State<AppState>,
    Protobuf(create_user): Protobuf<CreateUserRequest>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    create_user.validate()?;
    match state.save(create_user).await {
        Ok(id) => Ok((
            StatusCode::CREATED,
            [(header::LOCATION, format!("/users/{}", id))],
        )),
        Err(PersistenceError::UniqueViolation) => Err(StatusCode::UNPROCESSABLE_ENTITY),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
