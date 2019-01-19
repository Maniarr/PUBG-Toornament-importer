use super::super::util::jwt::Role;

#[table_name="users"]
#[derive(Serialize, Debug)]
pub struct User {
    pub id: Option<i64>,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub roles: Vec<Role>,
}

#[derive(Serialize, Deserialize)]
pub struct UserForm {
    pub username: String,
    pub password: String,
}

impl User {
}
