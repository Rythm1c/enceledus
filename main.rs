pub mod src;

fn main()->Result<(), Box<dyn std::error::Error>> {
    
    src::app::main_loop::run()?;
    Ok(())
 
}