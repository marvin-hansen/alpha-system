use std::error::Error;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum ServerError {
    UnknownServerError = 99, // 0 is reserved for success i.e. 202 for http
    TopicAlreadyExists = 1,
    StreamAlreadyExists = 2,
    UserAlreadyExists = 3,
    AccessTokenAlreadyExists = 4,
    TopicNotFound = 5,
    StreamNotFound = 6,
    UserNotFound = 7,
    AccessTokenNotFound = 8,
    InvalidCredentials = 9,
    InvalidTopicId = 10,
    InvalidStreamId = 11,
    InvalidAccessToken = 12,
    InvalidMessageId = 13,
    InvalidMessagePayload = 14,
}

impl Error for ServerError {}

impl From<ServerError> for u8 {
    #[inline]
    fn from(error: ServerError) -> u8 {
        error as u8
    }
}

impl From<u32> for ServerError {
    #[inline]
    fn from(value: u32) -> Self {
        Self::from(value as u8)
    }
}

impl From<u16> for ServerError {
    #[inline]
    fn from(value: u16) -> Self {
        Self::from(value as u8)
    }
}

impl From<u8> for ServerError {
    #[inline]
    fn from(error: u8) -> ServerError {
        match error {
            99 => ServerError::UnknownServerError,
            1 => ServerError::TopicAlreadyExists,
            2 => ServerError::StreamAlreadyExists,
            3 => ServerError::UserAlreadyExists,
            4 => ServerError::AccessTokenAlreadyExists,
            5 => ServerError::TopicNotFound,
            6 => ServerError::StreamNotFound,
            7 => ServerError::UserNotFound,
            8 => ServerError::AccessTokenNotFound,
            9 => ServerError::InvalidCredentials,
            10 => ServerError::InvalidTopicId,
            11 => ServerError::InvalidStreamId,
            12 => ServerError::InvalidAccessToken,
            13 => ServerError::InvalidMessageId,
            14 => ServerError::InvalidMessagePayload,
            _ => ServerError::UnknownServerError,
        }
    }
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::UnknownServerError => write!(f, "unknown server error"),
            ServerError::TopicAlreadyExists => write!(f, "topic already exists"),
            ServerError::StreamAlreadyExists => write!(f, "stream already exists"),
            ServerError::UserAlreadyExists => write!(f, "user already exists"),
            ServerError::AccessTokenAlreadyExists => write!(f, "access token already exists"),
            ServerError::TopicNotFound => write!(f, "topic not found"),
            ServerError::StreamNotFound => write!(f, "stream not found"),
            ServerError::UserNotFound => write!(f, "user not found"),
            ServerError::AccessTokenNotFound => write!(f, "access token not found"),
            ServerError::InvalidCredentials => write!(f, "invalid credentials"),
            ServerError::InvalidTopicId => write!(f, "invalid topic id"),
            ServerError::InvalidStreamId => write!(f, "invalid stream id"),
            ServerError::InvalidAccessToken => write!(f, "invalid access token"),
            ServerError::InvalidMessageId => write!(f, "invalid message id"),
            ServerError::InvalidMessagePayload => write!(f, "invalid message payload"),
        }
    }
}
