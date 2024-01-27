use crate::{
    api::{AppError, AppJson, BaseResponse},
    file::read_files,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct Comment {
    pub name: String,
    pub is_attend: bool,
    pub message: String,
}

pub async fn get_comments() -> Result<AppJson<BaseResponse<Vec<Comment>>>, AppError> {
    let extension = ".txt";
    let files = read_files("comments", extension)?;
    let mut comments: Vec<Comment> = Vec::with_capacity(files.len());

    for (f, content) in files {
        let f: Vec<_> = f[..f.len() - extension.len()]
            .split("##")
            .into_iter()
            .collect();

        if f.len() > 1 {
            comments.push(Comment {
                name: f[0].to_string(),
                is_attend: f[1].parse::<bool>().unwrap_or_default(),
                message: content,
            })
        }
    }

    Ok(AppJson(BaseResponse::success(Some(comments))))
}
