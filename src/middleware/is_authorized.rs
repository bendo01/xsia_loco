use std::{
    convert::Infallible,
    task::{Context, Poll},
};

use axum::{
    body::Body,
    extract::{FromRequestParts, Request},
    response::Response,
};
use futures_util::future::BoxFuture;
use loco_rs::prelude::{auth::JWTWithUser, *};
use tower::{Layer, Service};

use crate::models::auth::permission_user::_entities::permission_user as AuthPermissionUser;
use crate::models::auth::permissions::_entities::permissions as AuthPermission;
use crate::models::auth::users::_entities::users as AuthUser;

#[derive(Clone)]
pub struct AuthorizedLayer {
    state: AppContext,
    permission: String,
}

#[derive(Clone)]
pub struct AuthorizedService<S> {
    inner: S,
    state: AppContext,
    permission: String,
}

impl AuthorizedLayer {
    #[must_use]
    pub fn new(state: AppContext, permission: impl Into<String>) -> Self {
        Self {
            state,
            permission: permission.into(),
        }
    }
}

impl<S> Layer<S> for AuthorizedLayer {
    type Service = AuthorizedService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Self::Service {
            inner,
            state: self.state.clone(),
            permission: self.permission.clone(),
        }
    }
}

impl<S, B> Service<Request<B>> for AuthorizedService<S>
where
    S: Service<Request<B>, Response = Response<Body>, Error = Infallible> + Clone + Send + 'static,
    S::Future: Send + 'static,
    B: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let state = self.state.clone();
        let clone = self.inner.clone();
        let permission = self.permission.clone();

        let mut inner = std::mem::replace(&mut self.inner, clone);
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let auth = JWTWithUser::<AuthUser::Model>::from_request_parts(&mut parts, &state).await;

            match AuthUser::Model::find_by_email(&state.db, &auth.unwrap().user.email).await {
                Ok(user) => {
                    // Now 'user' is of type AuthUser::Model (not Option)
                    match AuthPermission::Entity::find()
                        .filter(AuthPermission::Column::DeletedAt.is_null())
                        .filter(AuthPermission::Column::Name.eq(&permission))
                        .one(&state.db)
                        .await
                    {
                        Ok(Some(access)) => {
                            match AuthPermissionUser::Entity::find()
                                .filter(AuthPermissionUser::Column::UserId.eq(user.id))
                                .filter(AuthPermissionUser::Column::PermissionId.eq(access.id))
                                .one(&state.db)
                                .await
                            {
                                Ok(Some(_)) => {
                                    // Reconstruct the request and add user to extensions
                                    let mut req = Request::from_parts(parts, body);
                                    req.extensions_mut().insert(user);
                                    inner.call(req).await
                                }
                                Ok(None) => Ok(Response::builder()
                                    .status(403)
                                    .body(Body::from("Forbidden: No permission"))
                                    .unwrap()),
                                Err(e) => Ok(Response::builder()
                                    .status(500)
                                    .body(Body::from(format!("Database error: {e}")))
                                    .unwrap()),
                            }
                        }
                        Ok(None) => Ok(Response::builder()
                            .status(403)
                            .body(Body::from("Forbidden: Permission not found"))
                            .unwrap()),
                        Err(e) => Ok(Response::builder()
                            .status(500)
                            .body(Body::from(format!("Database error: {e}")))
                            .unwrap()),
                    }
                }
                Err(_) => Ok(Response::builder()
                    .status(401)
                    .body(Body::from("Unauthorized: User not found"))
                    .unwrap()),
            }
        })
    }
}
