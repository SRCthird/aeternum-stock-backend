use diesel::result;

#[derive(Debug)]
pub enum AppError {
    DbError(result::Error),
}

impl From<result::Error> for AppError {
    fn from(err: result::Error) -> Self {
        AppError::DbError(err)
    }
}

impl From<AppError> for axum::Error {
    fn from(err: AppError) -> Self {
        match err {
            AppError::DbError(_e) => {
                axum::Error::new(std::io::Error::new(std::io::ErrorKind::Other, "Database error"))
            },
        }
    }
}
