use crate::HeaderMap;

pub fn authenticate(headers: HeaderMap) -> Option<String> {
    let user_id = headers.get("x-replit-user-id");
    user_id.map(|id| id.to_str().unwrap().to_string())
}
