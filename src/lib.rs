#[cfg(test)]
mod tests {
    use crate::js_types::{JsString, JsArray};

    #[test]
    fn general() {
        let st = JsString::new("hello, world! \"hello\" he said.".to_string()).build_string();
        let st2 = JsString::new("hello, world! \"hello\" he said.".to_string()).build_string();
        let st3 = JsString::new("hello, world! \"hello\" he said.".to_string()).build_string();
        let arr = JsArray::new(Some(vec![st, st2, st3]));
        println!("{}", arr.build_array());
    }
}

pub mod js_types;