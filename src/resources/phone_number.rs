use crate::error::Result;
use crate::http::{enc, HttpClient};
use crate::types::{
    CreatePhoneNumberParams, ImportPhoneNumberParams, PhoneNumber, UpdatePhoneNumberParams,
};
use reqwest::Method;

/// Phone number operations (`client.phone_number()`).
#[derive(Clone)]
pub struct PhoneNumberClient {
    pub(crate) http: HttpClient,
}

impl PhoneNumberClient {
    /// `POST /create-phone-number` — purchases a Retell-managed number (billing required).
    pub async fn create(&self, params: &CreatePhoneNumberParams) -> Result<PhoneNumber> {
        self.http
            .send_json(Method::POST, "/create-phone-number", params)
            .await
    }

    /// `POST /import-phone-number`
    pub async fn import(&self, params: &ImportPhoneNumberParams) -> Result<PhoneNumber> {
        self.http
            .send_json(Method::POST, "/import-phone-number", params)
            .await
    }

    /// `GET /get-phone-number/{phone_number}`
    pub async fn get(&self, phone_number: &str) -> Result<PhoneNumber> {
        self.http
            .send_empty(
                Method::GET,
                &format!("/get-phone-number/{}", enc(phone_number)),
            )
            .await
    }

    /// `GET /v2/list-phone-numbers`
    pub async fn list(&self) -> Result<Vec<PhoneNumber>> {
        self.http
            .send_empty(Method::GET, "/v2/list-phone-numbers")
            .await
    }

    /// `PATCH /update-phone-number/{phone_number}`
    pub async fn update(
        &self,
        phone_number: &str,
        params: &UpdatePhoneNumberParams,
    ) -> Result<PhoneNumber> {
        self.http
            .send_json(
                Method::PATCH,
                &format!("/update-phone-number/{}", enc(phone_number)),
                params,
            )
            .await
    }

    /// `DELETE /delete-phone-number/{phone_number}`
    pub async fn delete(&self, phone_number: &str) -> Result<()> {
        self.http
            .send_no_content(
                Method::DELETE,
                &format!("/delete-phone-number/{}", enc(phone_number)),
            )
            .await
    }
}
