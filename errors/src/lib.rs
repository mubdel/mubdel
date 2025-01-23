use std::fs;

use async_graphql::{Error, ErrorExtensions, Result};
use fluent::{FluentArgs, FluentValue};
use fluent_bundle::bundle::FluentBundle;
use fluent_bundle::FluentResource;
use intl_memoizer::concurrent::IntlLangMemoizer;
use unic_langid::langid;

pub mod errors;

#[macro_export]
macro_rules! gql_err {
    ($diag_ctx:expr, $err:expr) => {
        let r = $diag_ctx.new_err($err);
        return Err(r);
    };
}

pub struct Err<'a> {
    code: &'a str,
    message: String,
    details: String,
    suggestion: String,
    fields: String,
    category: String,
    http_code: u16,
    // TODO: request id, doc_url
}

impl Err<'_> {
    pub fn to_err(&self) -> Error {
        Error::new(&self.message).extend_with(|_, e| {
            e.set("code", self.code);
            if !self.details.is_empty() {
                e.set("details", &self.details);
            }
            if !self.suggestion.is_empty() {
                e.set("suggestion", &self.suggestion);
            }
            if !self.fields.is_empty() {
                let s: String = self.fields.split_whitespace().collect();
                let fields: Vec<&str> = s.split(',').collect();
                e.set("fields", fields);
            }
            if !self.category.is_empty() {
                e.set("category", &self.category);
            }
            if self.http_code != 0 {
                e.set("httpCode", self.http_code);
            }
        })
    }
}

pub struct DiagCtx {
    bundle: FltBundle,
}

impl DiagCtx {
    pub fn new(bundle: FltBundle) -> Self {
        Self { bundle }
    }

    pub fn new_err<'a>(&self, err: impl IntoDiagnostic<'a>) -> Error {
        let builder = err.to_diagnostic();
        let vars = err.get_struct_info();

        builder.create_err(&self.bundle, vars).to_err()
    }
}

pub struct DiagnosticBuilder<'a> {
    ftl: &'a str,
    code: &'a str,
    category: &'a str,
    http_code: u16,
}

impl<'a> DiagnosticBuilder<'a> {
    pub fn new(ftl: &'a str, code: &'a str, category: &'a str, http_code: u16) -> Self {
        Self {
            ftl,
            code,
            category,
            http_code,
        }
    }

    pub fn ftl(&self) -> &str {
        self.ftl
    }

    pub fn code(&self) -> &str {
        self.code
    }

    pub fn category(&self) -> &str {
        self.category
    }

    pub fn http_code(&self) -> u16 {
        self.http_code
    }

    pub fn create_err(&self, bundle: &FltBundle, vars: Vec<(&str, &str)>) -> Err {
        let mut args = FluentArgs::new();
        for (var, value) in vars.into_iter() {
            args.set(var, FluentValue::from(value));
        }

        let msg = bundle.get_message(self.ftl).unwrap();
        let pattern = msg.value().unwrap();
        let mut errs = vec![];
        let message = bundle
            .format_pattern(pattern, Some(&args), &mut errs)
            .to_string();

        let (mut details, mut suggestion, mut fields) =
            (String::new(), String::new(), String::new());

        msg.attributes().for_each(|attr| {
            let attr_name = attr.id();
            match attr_name {
                "details" => {
                    details = bundle
                        .format_pattern(attr.value(), Some(&args), &mut errs)
                        .to_string()
                }
                "suggestion" => {
                    suggestion = bundle
                        .format_pattern(attr.value(), Some(&args), &mut errs)
                        .to_string()
                }
                "fields" => {
                    fields = bundle
                        .format_pattern(attr.value(), Some(&args), &mut errs)
                        .to_string()
                }
                _ => unimplemented!(),
            }
        });

        Err {
            code: self.code,
            message,
            details,
            suggestion,
            fields,
            category: self.category.to_string(),
            http_code: self.http_code,
        }
    }
}

pub trait IntoDiagnostic<'a> {
    fn to_diagnostic(&self) -> DiagnosticBuilder<'a>;
    fn get_struct_info(&self) -> Vec<(&str, &str)>;
}

pub type FltBundle = FluentBundle<FluentResource, IntlLangMemoizer>;

pub fn fluent_messages(fluent_path: String) -> Result<FltBundle> {
    let mut bundle: FltBundle = FluentBundle::new_concurrent(vec![langid!("en-US")]);

    let ftl_str = fs::read_to_string(fluent_path)?;

    let resource = FluentResource::try_new(ftl_str).expect("Failed to parse an FTL string");

    bundle
        .add_resource(resource)
        .expect("Failed to add FTL resources to the bundle");

    bundle.set_use_isolating(false);

    Ok(bundle)
}
