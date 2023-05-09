use std::sync::{Arc, Mutex};
use std::time::Duration;

use rocket::futures::TryFutureExt;
use rocket::tokio;
use rocket::tokio::task::JoinHandle;
use rocket::tokio::time::sleep;

use crate::core::services::authz_service::AuthzService;
use crate::core::shared::authz_card::AuthzCard;
use crate::models::errors::custom::CustomError;

pub struct AuthzServiceImpl {
    authorizations: Arc<Mutex<Vec<AuthzCard>>>,
    thread: JoinHandle<Result<bool, CustomError>>
}

impl AuthzServiceImpl {
    pub async fn new (authz_url: String) -> Self {

        let authorizations_arc = Arc::new(Mutex::new(vec![]));

        let th: JoinHandle<_> = tokio::spawn( {

            let cloned_authz = Arc::clone(&authorizations_arc);
            let url = authz_url.clone();
            async move {
                loop {
                    println!("sync with authz api");
                    // je ne fais pas un await? car si authz ne rep pas 1 a ce moment, ca KO le thread

                    let old_authz = cloned_authz.lock()
                        .unwrap()
                        .clone()
                        .into_iter()
                        .collect::<Vec<_>>();

                    let mut new_data = Self::get_authorizations_from_api(url.as_str()).await.unwrap_or(old_authz);
                    cloned_authz.lock().unwrap().clear();
                    cloned_authz.lock().unwrap().append(&mut new_data);
                    sleep(Duration::from_secs(10 * 60)).await;
                }
            }
        });

        Self { authorizations: authorizations_arc, thread: th }
    }

    async fn get_authorizations_from_api(url_authz: &str) -> Result<Vec<AuthzCard>, CustomError> {
        let resource = "urn:api:authn";
        reqwest::get(format!("{}/authz_cards_by_resource/{}", url_authz, resource))
            .and_then(|response| {
                response.json::<Vec<AuthzCard>>()
            })
            .await
            .map_err(|err| CustomError::new(format!("{}", err).as_str()))
    }
}

impl Drop for AuthzServiceImpl {
    fn drop(&mut self) {
        println!("authz : {:?}", self.authorizations);
        self.thread.abort();
    }
}

impl AuthzService for AuthzServiceImpl {
    fn evaluate(&self, resource: &str, action: &str, pseudo: &str) -> Result<bool, CustomError> {
        self.authorizations.lock().unwrap().iter()
            .find(|card| card.resource.as_str() == resource && card.action == action && card.users.contains(&pseudo.to_string()))
            .map(|_| Ok(true))
            .unwrap_or(Err(CustomError::new("not authorised")))
    }
}