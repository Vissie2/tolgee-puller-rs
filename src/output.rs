use crate::tolgee::Resources;
use std::{fs::File, io::Write};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OutputError {
    #[error("Failed formatting JSON.")]
    ReadJSON(#[from] serde_json::Error),
    #[error("Failed creating or writing to file.")]
    File(#[from] std::io::Error),
}

fn resources_to_code_str(resources: &Resources) -> Result<String, OutputError> {
    let json_str = serde_json::to_string_pretty(&resources)?;
    Ok(format!("// THIS FILE IS GENERATED, DO NOT EDIT!\nconst resources = {json_str};\ntype Resources = typeof resources;\nexport {{ resources, type Resources }};"))
}

pub fn write_resources_to_file(
    resources: Resources,
    output_path: String,
    split: bool,
) -> Result<(), OutputError> {
    if split {
        todo!();
    }

    let mut new_file = File::create(output_path)?;
    let code = resources_to_code_str(&resources)?;
    new_file.write_all(code.as_bytes())?;

    Ok(())
}
