pub fn suffixer(word: &str, suffix: &str) -> Result<String, u8>
{
    let new_string = format!("{0}{1}", word, suffix);
    let result: Result<String, u8> = Ok(new_string);
    result
}