use super::super::{
    model::{user},
    util::response::*
};

use rocket::{
    http::{ContentType, Status},
    Route
};
use rocket_contrib::json::Json;

pub fn register_routes() -> Vec<Route> {
    routes![]
}
