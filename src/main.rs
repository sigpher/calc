use crate::hundred_problems::run;

mod hundred_problems;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("正在生成数学口算题");
    run()?;
    Ok(())
}
