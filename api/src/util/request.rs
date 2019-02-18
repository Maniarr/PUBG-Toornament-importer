use rocket::request::FromParam;
use rocket::http::RawStr;
use std::str::FromStr;
use uuid::Uuid;

pub struct UuidParam {
    pub uuid: Uuid
}

impl<'r> FromParam<'r> for UuidParam {
    type Error = &'r RawStr;

    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
        match Uuid::from_str(param) {
            Ok(uuid) => Ok(UuidParam {
                uuid
            }),
            Err(_) => Err(param)
        }
    }
}
