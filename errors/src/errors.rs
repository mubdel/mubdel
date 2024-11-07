use error_macros::Diagnostic;

use crate::{DiagnosticBuilder, IntoDiagnostic};

#[derive(Diagnostic)]
#[diag(password_not_match)]
#[code(E0001)]
#[category(bad_user_input)]
#[http_code(400)]
pub struct PasswordNotMatch<'a> {
    pub field1: &'a str,
    pub field2: &'a str,
}

#[derive(Diagnostic)]
#[diag(invalid_credential)]
#[code(E0002)]
#[category(bad_user_input)]
#[http_code(400)]
pub struct InvalidCredential<'a> {
    pub field1: &'a str,
    pub field2: &'a str,
}

#[derive(Diagnostic)]
#[diag(unauthorized)]
#[code(E0003)]
#[category(unauthorized)]
#[http_code(401)]
pub struct Unauthorized {}

#[derive(Diagnostic)]
#[diag(auth_link_used_expired)]
#[code(E0004)]
#[category(unauthorized)]
#[http_code(401)]
pub struct AuthLinkUsedExpired {}
