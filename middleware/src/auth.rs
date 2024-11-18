use chrono::Utc;
use poem::{
    web::headers::{Cookie, HeaderMapExt},
    Endpoint, IntoResponse, Request, Response, Result,
};
use tracing::{error, info};

use helper::poem::{get_cache, get_db};
use ty::{
    auth::SessionSecret,
    role::{Role, UserRole},
    user::UserStatus,
};

#[macro_export]
macro_rules! auth_next {
    ($req:expr, $next:expr) => {
        $req.extensions_mut().insert(Role::Guest);

        let res = $next.call($req).await;

        return match res {
            Ok(response) => Ok(response.into_response()),
            Err(err) => Err(err),
        };
    };
}

pub async fn authenticator<E: Endpoint>(next: E, mut req: Request) -> Result<Response> {
    let cookies = req.headers().typed_get::<Cookie>();
    if cookies.is_none() {
        info!("empty `Cookie` header");
        auth_next!(req, next);
    }
    let cookies = cookies.unwrap();

    let cookie = cookies.get("mdl_session");
    if cookie.is_none() {
        info!("`mdl_session` cookie not found");
        auth_next!(req, next);
    }
    let session_secret = cookie.unwrap().to_string();

    let r = get_cache(&req);
    if r.is_err() {
        error!("get cache: {:?}", r.err());
        auth_next!(req, next);
    }
    let cache = r.clone().unwrap();

    let session = cache
        .session_cache()
        .get_session_by_secret(session_secret.clone())
        .await;
    if session.is_err() {
        info!("session not found");
        auth_next!(req, next);
    }
    let session = session.unwrap();
    if Utc::now() > session.expires_on {
        info!("session not expired");
        auth_next!(req, next);
    }

    let r = get_db(&req);
    if r.is_err() {
        error!("get database: {:?}", r.err());
        auth_next!(req, next);
    }
    let db = r.unwrap();

    let r = db.user_db().get_user(session.user).await;
    if r.is_err() {
        error!("get user: {:?}", r.err());
        auth_next!(req, next);
    }
    let user = r.unwrap();
    if user.deleted_at.is_some()
        || !user.email_verified
        || !matches!(user.status, UserStatus::Active)
    {
        info!("user deleted, not active or email not verified");
        auth_next!(req, next);
    }

    req.extensions_mut()
        .insert(SessionSecret(Some(session_secret)));

    let role = Role::User(UserRole {
        id: user.id,
        username: user.username,
    });
    req.extensions_mut().insert(role);

    let res = next.call(req).await;
    match res {
        Ok(response) => Ok(response.into_response()),
        Err(err) => Err(err),
    }
}
