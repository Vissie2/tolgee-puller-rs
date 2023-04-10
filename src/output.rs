use crate::tolgee::Resources;
use std::{fs::File, io::Write};

fn resources_to_code_str(resources: &Resources) -> Result<String, String> {
    let json_str_result = serde_json::to_string_pretty(&resources);

    let json_str = match json_str_result {
        Ok(json_str) => json_str,
        Err(_) => return Err("Couldn't convert to JSON.".to_string()),
    };

    Ok(format!("// THIS FILE IS GENERATED, DO NOT EDIT!\nconst resources = {json_str};\ntype Resources = typeof resources;\nexport {{ resources, type Resources }};"))
}

pub fn write_resources_to_file(
    resources: Resources,
    output_path: String,
    split: bool,
) -> Result<(), String> {
    if split {
        todo!();
    }

    let mut new_file = match File::create(output_path) {
        Ok(file) => file,
        Err(_) => return Err("Couldn't create file.".to_string()),
    };

    let code = match resources_to_code_str(&resources) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let write_result = new_file.write(code.as_bytes());

    if write_result.is_err() {
        return Err("Couldn't write to file.".to_string());
    }

    Ok(())
}
