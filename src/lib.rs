#[cfg(test)]
mod tests {
    use crate::js::{js_import::JsImport, js_types::JsString};

    #[test]
    fn a() {
        let import = JsImport::from_string_imports(vec!["something".to_string(), "foo".to_string(), "bar".to_string()], JsString::new("\"discord.js\"".to_string()));
        println!("{}", import.build_import());
    }
}

pub mod js;
pub mod discord;