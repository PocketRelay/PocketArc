use serde::Serialize;

#[derive(Serialize)]
pub struct Currency {
    pub name: String,
    pub balance: u32,
}

#[derive(Serialize)]
pub struct UserCurrenciesResponse {
    pub list: Vec<Currency>,
}
