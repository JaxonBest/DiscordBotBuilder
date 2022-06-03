/// JS Parsable String.
pub struct JsString {
    pub content: String
}

impl JsString {
    pub fn new(content: String) -> Self {
        Self { content }
    }

    /// Converts the JsString's content into a usable string in Javascript.
    pub fn build_string(&self) -> String {
        let inner = self.content.replace('"', "\\\""); // Replaces " with \"
        format!("\"{}\"", inner)
    }
}

/// Build an array from string elements. To use Javascript enforcing types choose the
/// right struct for example: String = `JsString` then build into a String.
pub struct JsArray {
    pub elements: Vec<String>
}

impl JsArray {
    pub fn new(elements: Option<Vec<String>>) -> Self {
        match elements {
            Some(r) => {
                Self { elements: r }
            }
            None => {
                Self { elements: Vec::new() }
            }
        }
    }

    /// Create a string containing the syntax of Javascript and elements from the JsArray.elements.
    /// ## Example
    /// ```
    /// let mut js_arr = JsArray::new(None);
    /// let index_one = JsString::new("Hello, world!".to_string()).build_string();
    /// js_arr.push(index_one);
    /// println!("{}", js_arr.build_array()) // ["Hello, world!"]
    /// ```
    pub fn build_array(&self) -> String {
        let mut index = 0;
        let mut str_arr = String::new();

        for el in &self.elements {
            let mut start_str = "";
            if index < el.len() {
                start_str = ", ";
            }
            str_arr.push_str(&format!("{}{}", el, start_str));

            index += 1;
        }

        format!("[{}]", str_arr)
    }
}