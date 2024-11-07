use anyhow::Result;
use handlebars::{DirectorySourceOptions, Handlebars};

pub fn register_templates<'a>(tmpl_path: &str) -> Result<Handlebars<'a>> {
    let mut tmpl = Handlebars::new();

    tmpl.register_templates_directory(
        tmpl_path,
        DirectorySourceOptions {
            tpl_extension: ".tmpl".to_string(),
            hidden: false,
            temporary: false,
        },
    )?;

    Ok(tmpl)
}
