use crate::model;
use warp::reply::json;

pub async fn version_info() -> model::WebResult<impl warp::Reply> {
    let response_json = &model::VersionInfoResponse {
        status: "Success".to_string(),
        version: "pokemon-api v0.0.1".to_string(),
    };
    Ok(json(response_json))
}
