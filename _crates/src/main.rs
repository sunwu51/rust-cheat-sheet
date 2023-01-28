
mod val;
mod ah;
mod te;
fn main() -> anyhow::Result<()> {
    println!("{:?}", ah::f2()?);
    Ok(())
}
