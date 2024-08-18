use std::error::Error;
use color_eyre::Result;


fn translate_two() -> Result<(), Box<dyn Error>> {
    println!("translate two");
    Ok(())
}

async fn translate_one() -> Result<(), Box<dyn Error>> {
    Ok(())
}

async fn translate_expr() -> Result<(), Box<dyn Error>> {
    println!("translate expr");
    translate_two()?;
    Ok(())
}

fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(translate_expr());
}