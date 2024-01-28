use std::time::Duration;

use crate::{
    api::{ApiResponse, BaseResponse},
    AppState,
};
use anyhow::anyhow as AnyErr;
use axum::{
    extract::{Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use tokio::{stream, time::sleep};

#[derive(Serialize)]
pub struct Comment {
    pub name: String,
    pub is_attend: bool,
    pub message: String,
}

#[derive(Serialize)]
pub struct GetComments {
    pub total: i32,
    pub attend: i32,
    pub skip: i32,
    pub comments: Vec<Comment>,
}

pub async fn get_comments(state: State<AppState>) -> ApiResponse<GetComments> {
    let extension = ".txt";
    let files = state.mutex.read().await.read_files("comments", extension)?;
    let mut comments: Vec<Comment> = Vec::with_capacity(files.len());
    let (mut total, mut attend, mut skip) = (0, 0, 0);

    for (f, content) in files {
        let f: Vec<_> = f[..f.len() - extension.len()]
            .split("##")
            .into_iter()
            .collect();

        if f.len() > 1 {
            let is_attend = f[1].parse::<bool>().unwrap_or_default();
            total += 1;

            match is_attend {
                true => attend += 1,
                false => skip += 1,
            }

            comments.push(Comment {
                name: f[0].to_string(),
                is_attend: is_attend,
                message: content,
            })
        }
    }

    Ok(BaseResponse::success(Some(GetComments {
        total: total,
        attend: attend,
        skip: skip,
        comments: comments,
    })))
}

#[derive(Deserialize)]
pub struct CreateComment {
    pub name: String,
    pub is_attend: bool,
    pub message: String,
}

pub async fn create_comment(state: State<AppState>, req: Json<CreateComment>) -> ApiResponse<()> {
    if req.message.len() > 300 {
        return Err(AnyErr!("failed to create, max message is 300 chars").into());
    }

    let filepath = format!("{}/{}##{}.txt", "comments", req.name, req.is_attend);
    let _ = state
        .mutex
        .write()
        .await
        .insert_to_file(&filepath, &req.message)?;

    Ok(BaseResponse::success(None))
}

#[derive(Deserialize)]
pub struct DeleteCommentQuery {
    pub id: String,
}

pub async fn delete_comment(
    state: State<AppState>,
    q: Query<DeleteCommentQuery>,
) -> ApiResponse<()> {
    let file_path = format!("{}/{}.txt", "comments", q.id.replace("%%", "##"));
    state.mutex.write().await.delete_file(&file_path);
    Ok(BaseResponse::success(None))
}
