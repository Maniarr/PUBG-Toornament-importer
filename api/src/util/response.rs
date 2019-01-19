use serde::Serialize;
use rocket::{
    http::{ContentType, Status},
    request::Request,
    response::{self, Response, Responder}
};
use std::io::Cursor;

#[derive(Serialize)]
pub struct CustomError {
    pub message: String
}

#[derive(Serialize)]
pub enum JsonResponse<T, E> where T: Serialize, E: Serialize {
    Ok(u16, T),
    Err(u16, E)
}

impl<'r, T: Serialize, E: Serialize> Responder<'r> for JsonResponse<T, E> {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        let (status_code, json) = match self {
            JsonResponse::Ok(status, object) => (status, serde_json::to_string(&object).expect("error convert struct")),
            JsonResponse::Err(status, object) => (status, serde_json::to_string(&object).expect("error convert struct"))
        };

        Response::build()
            .status(Status::from_code(status_code).expect("Response code doesn't exist"))
            .header(ContentType::JSON)
            .sized_body(Cursor::new(json))
            .ok()
    }
}

impl<'r> rocket::response::Responder<'r> for CustomError {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .header(ContentType::Plain)
            .sized_body(Cursor::new(self.message))
            .ok()
    }
}
