const OUTPUT_PATH: &str = "blocks";
const STANDARD_BLOCKS_URL: &str = "https://github.com/rusty-scratch/scratch-block-definition";

#[cfg(not(feature = "blocks"))]
fn main() {
    panic!("Feature `blocks` not enabled");
}

#[cfg(feature = "blocks")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use git2::Repository;
    Repository::clone(
        STANDARD_BLOCKS_URL,
        format!("{OUTPUT_PATH}/scratch-standard-blocks"),
    )?;
    Ok(())
}
