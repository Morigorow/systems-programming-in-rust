use std::error::Error;

use reqwest;


fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://www.rust-lang.org";
    let mut response = reqwest::get(url)?;
    
    let content = response.text()?;
    print!("{}", content);

    Ok(())
}
