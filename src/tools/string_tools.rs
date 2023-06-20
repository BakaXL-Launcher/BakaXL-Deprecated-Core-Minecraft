use std::collections::HashMap;

pub fn replace_variables(input_string: &str, variables: &HashMap<&str, String>) -> String {
    let mut replaced_string = String::from(input_string);

    for (variable_name, variable_value) in variables {
        let placeholder = format!("${{{}}}", variable_name);
        replaced_string = replaced_string.replace(&placeholder, variable_value);
    }

    replaced_string
}