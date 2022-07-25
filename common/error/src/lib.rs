use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum AppError {
    // 引数が不正である
    #[error("{0}")]
    InvalidArgument(String),

    // エンティティが存在しない
    #[error("{0}")]
    NotFound(String),

    // データベースへの接続失敗などの技術的問題
    #[error("{0}")]
    Internal(String),
}
