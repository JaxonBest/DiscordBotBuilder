/// Represent a variable in Javascript.
/// 
/// **NOTE: If you want to build the variable it does require mutability.**
pub struct JsVar {
    name: String,
    value: String
}

impl JsVar {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }

    /// Build the struct into a working variable format in Javascript.
    /// 
    /// ## Example
    /// ``` rs
    /// let a = JsVar::new(
    ///     "hello world!".to_string(), // Name will be translated to "_hello_world__" 
    ///     JsString::new("Hello there! This is a value..".to_string()).build_string());
    ///
    /// println!("{}", a.build_var());
    /// ```
    pub fn build_var(&mut self) -> String {
        self.secure_name();
        format!("{} = {}", self.name, self.value)
    }

    pub fn is_name_safe(&self) -> bool {
        let keywords = [ // There's a lot
            "await", "break", "case",
            "catch", "class", "const",
            "continue", "debugger", "do",
            "debugger", "do", "else", 
            "false", "for", "function",
            "if", "import", "in",
            "instanceof", "let", "new",
            "null", "return", "switch",
            "throw", "try", "true",
            "typeof", "var", "while"
        ];

        if keywords.contains(&self.name.as_str()) || self.name.starts_with(" ") || self.name.ends_with(" ") {
            return false
        }

        true
    }

    /// Makes the name of the variable safe to use in Javascript. 
    /// ## Example
    /// *hello world* into *\_hello_world_*
    pub fn secure_name(&mut self) {
        for ch in "~`!@#$%^&*()-=|\\][';:<,>./? ".chars() {
            self.name = self.name.replace(ch, "_");
        }

        self.name = format!("_{}_", self.name);
    }

    /// Makes the name of the variable safe to use in Javascript.
    ///
    /// But returns the result instead of affecting `self`.
    /// ## Example
    /// *hello world* into *\_hello_world_*
    pub fn get_secure_name(&self) -> String {
        let mut name = self.name.clone();
        for ch in "~`!@#$%^&*()-=|\\][';:<,>./? ".chars() {
            name = name.replace(ch, "_");
        }

        format!("_{}_", name)
    }
}