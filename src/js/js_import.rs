use super::js_types::JsString;

pub struct JsImport {
    defaults: bool,
    importing: Vec<JsImportChild>,
    from: JsString
}

impl JsImport {
    pub fn new(defaults: bool, importing: Vec<JsImportChild>, from: JsString) -> Self {
        Self { defaults, importing, from: from }
    }

    pub fn from_string_imports(importing: Vec<String>, from: JsString) -> Self {
        let mut imports = Vec::<JsImportChild>::new();
        for im in importing {
            imports.push(JsImportChild { default: false, name: im });        
        }
        Self { importing: imports, from, defaults: false }
    }

    pub fn new_default(call_name: String, from: JsString) -> Self {
        Self { importing: vec![JsImportChild::new_default(call_name)], defaults: true, from }
    }

    /// `const { i,m,p,o,r,t,s } = require('from');`
    /// or `const imports = require('from');`
    pub fn build_import(&self) -> String {
        let first = self.importing.get(0);
        match first {
            Some(r) => {
                if r.default {
                    return format!("const {} = require({})", r.name, self.from.build_string())
                }
            },
            None => {
                return format!("const {{ }} = require({})", self.from.build_string())
            }
        }

        let mut imports = String::new();
        for import in &self.importing {
            imports.push_str(&format!("{}, ", import.name));
        }

        format!("const {{ {}}} = require({})", imports, self.from.build_string())
    }
}

pub struct JsImportChild {
    default: bool,
    name: String
}

impl JsImportChild {
    pub fn new(name: String) -> Self {
        Self { default: false, name }
    }

    pub fn new_default(name: String) -> Self {
        Self { default: true, name }
    }

    pub fn set_default(&mut self, status: bool) {
        self.default = status;
    }
}