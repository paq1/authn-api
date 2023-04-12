
use mongodb::bson::{doc, Document};
use mongodb::Collection;
use mongodb::results::InsertOneResult;
use crate::api::components::mongo_component::ClientMongoComponent;
use crate::core::entities::user::User;
use crate::core::services::user_repository::UserRepository;
use crate::models::errors::custom::CustomError;
use rocket::futures::{TryFutureExt, TryStreamExt};
use crate::api::entities::user_dbo::UserDbo;
use mongodb::error::Error;

pub struct UserRepositoryMongo {
    pub collection: Collection<Document>
}

impl ClientMongoComponent for UserRepositoryMongo {}

#[async_trait]
impl UserRepository<User, Result<InsertOneResult, CustomError>> for UserRepositoryMongo {
    async fn insert_user(&self, user: User) -> Result<InsertOneResult, CustomError> {
        if self.exist(&user).await {
            Err(CustomError::new("Ce pseudo exist déjà."))
        } else {
            self.insert_user_without_check(&user).await
        }
    }

    async fn fetch_many(&self) -> Vec<User> {
        match self.find_all().await {
            Ok(users) => users,
            _ => {
                eprintln!("une erreur est survenue lors de la lecture");
                vec![]
            }
        }
    }

    async fn fetch_one_by_pseudo(&self, pseudo: String) -> Result<User, CustomError> {
        self.collection
            .find_one(
                Some(
                    doc! {
                        "pseudo": pseudo.as_str()
                    }
                ),
                None
            )
            .await
            .map(|dbo_doc_opt|{
                dbo_doc_opt
                    .map(|dbo_doc| {
                        let user_dbo: UserDbo = dbo_doc.into();
                        let user: User = user_dbo.into();
                        Ok(user)
                    })
                    .unwrap_or(Err(CustomError::new("impossible de recupere le joueur")))
            })
            .unwrap_or_else(|err| Err(CustomError::new(format!("{}", err.to_string()).as_str())))
    }
}

impl UserRepositoryMongo {
    async fn insert_user_without_check(&self, user: &User) -> Result<InsertOneResult, CustomError> {
        let document: Document = user.clone().into();
        self.collection
            .insert_one(document, None)
            .map_err(|_| CustomError::new("erreur lors de l'insertion en base"))
            .await
    }

    async fn exist(&self, user: &User) -> bool {
        self.fetch_one_by_pseudo(user.pseudo.clone())
            .await
            .is_ok()
    }

    async fn find_all(&self) -> Result<Vec<User>, Error> {
        Ok(
            self.collection
                .find(None, None)
                .await?
                .try_collect::<Vec<Document>>()
                .await?
                .into_iter()
                .map(|doc| doc.into())
                .map(|dbo: UserDbo| dbo.into())
                .collect::<Vec<User>>()
                .into()
        )
    }

    pub async fn new() -> Result<Self, mongodb::error::Error> {
        Ok(
            Self {
                collection: {
                    Self::collection_user().await?
                }
            }
        )
    }
}