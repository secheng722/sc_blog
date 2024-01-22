use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
//要求范型T实现Serialize trait
pub struct ApiResponse<T: Serialize> {
    code: i32,
    msg: String,
    //当为None时，不序列化该字段
    //如果是vec<T>
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn new(code: i32, msg: &str, data: Option<T>) -> Self {
        ApiResponse {
            code,
            msg: msg.to_string(),
            data,
        }
    }
    pub fn ok(data: T) -> Json<Self> {
        Json(Self::new(200, "ok", Some(data)))
    }
    pub fn ok_data() -> Json<Self> {
        Json(Self::new(200, "ok", None))
    }
    pub fn ok_msg(msg: &str) -> Json<Self> {
        Json(Self::new(200, msg, None))
    }
    pub fn ok_msg_data(msg: &str, data: T) -> Json<Self> {
        Json(Self::new(200, msg, Some(data)))
    }

    pub fn err(msg: &str) -> Json<Self> {
        Json(Self::new(-1, msg, None))
    }
}
