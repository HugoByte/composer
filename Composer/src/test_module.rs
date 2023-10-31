use super::*;

impl Composer {
    pub fn generate_main_file_code(&self) -> String {
        let mut main_file = format!(
            "\
use serde_json::Value;

macro_rules! make_struct {{
    ($name:ident, [$($visibality:vis $field_name:ident: $field_type:ty),*]) => {{
            struct $name {{
                $($visibality $field_name: $field_type, )*
            }}
    }};
}}

"
        );

        for (task_name, task) in self.tasks.borrow().iter() {
            let input_data = self.get_task_input_data(task_name, &task.input_args);
            main_file = format!(
                "\
{main_file}
make_struct!({input_data});
make_struct!({}, [action_name: String, pub input: {}Input, pub output: Value]);
",
                task.action_name, task.action_name
            );
        }

        main_file = format!(
            "\
{main_file}

fn main() {{}}

#[test]
fn generated_structs_test() {{
    let a1 = Action1Input {{
        url: \"http\".to_string(),
        era: 1,
        address: \"aurras\".to_string(),
        owner_key: \"ow234bdn234ciouwndfuwbfo456wefc\".to_string(),
    }};

    let _ = Action1{{
        action_name : \"Action1\".to_string(),
        input : a1,
        output : serde_json::to_value(\"sample output\").unwrap(), 
    }};

    let a2 = Action2Input {{
        url: \"http\".to_string(),
        era: 1,
        address: \"aurras\".to_string(),
        owner_key: \"ow234bdn234ciouwndfuwbfo456wefc\".to_string(),
    }};

    let _ = Action2{{
        action_name : \"Action2\".to_string(),
        input : a2,
        output : serde_json::to_value(\"sample output\").unwrap(), 
    }};

    let a3 = Action3Input {{
        url: \"http\".to_string(),
        era: 1,
        address: \"aurras\".to_string(),
        owner_key: \"ow234bdn234ciouwndfuwbfo456wefc\".to_string(),
    }};

    let _ = Action3{{
        action_name : \"Action3\".to_string(),
        input : a3,
        output : serde_json::to_value(\"sample output\").unwrap(), 
    }};

    let a4 = Action4Input {{
        url: \"http\".to_string(),
        era: 1,
        address: \"aurras\".to_string(),
        owner_key: \"ow234bdn234ciouwndfuwbfo456wefc\".to_string(),
    }};

    let _ = Action4{{
        action_name : \"Action4\".to_string(),
        input : a4,
        output : serde_json::to_value(\"sample output\").unwrap(), 
    }};

    let a5 = Action5Input {{
        url: \"http\".to_string(),
        era: 1,
        address: \"aurras\".to_string(),
        owner_key: \"ow234bdn234ciouwndfuwbfo456wefc\".to_string(),
    }};

    let _ = Action5{{
        action_name : \"Action5\".to_string(),
        input : a5,
        output : serde_json::to_value(\"sample output\").unwrap(), 
    }};
}}
        
"
        );

        main_file
    }
}
