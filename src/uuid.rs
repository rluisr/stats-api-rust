use rocket::{State};
use rocket_contrib::json::Json;
use mysql;
use mysql::{params};
use crate::models::Response;

#[derive(Serialize, Deserialize, Debug)]
pub struct UUID {
    pub uuid: String,
    pub app_name: String,
}

impl UUID {
    pub fn register(uuid: Json<UUID>, pool: State<mysql::Pool>) -> Response {
        let params = params!{
            "uuid" => &uuid.uuid
        };

        let response;
        let insert = format!("INSERT INTO {} (uuid) VALUES (:uuid)", uuid.app_name);
        let result = pool.prep_exec(insert, params);
        match result {
            Ok(_) => response = Response {
                status: "ok".to_string(),
            },
            // Todo エラーハンドリングのやり方と、失敗したときのエラーレスポンスをちゃんとしたい
            Err(_) => response = Response {
                status: "ng".to_string(),
            }
        };

        response
    }
}