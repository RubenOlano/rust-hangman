mod game;
fn main() -> Result<(), game::GameErr> {
    game::run()?;
    Ok(())
}
