use std::{env, fs, path::Path};

/// mm hm
/// just gonna yoink this
#[path = "src/block_definer.rs"]
mod block_definer;

const BLOCKS_PATH: &str = "blocks";

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    #[cfg(feature = "block-definer")]
    define_blocks(out_dir);
}

#[cfg(feature = "block-definer")]
fn define_blocks<P: AsRef<Path>>(out_dir: P) {
    match fs::create_dir(out_dir.as_ref().join("blocks")) {
        Ok(_) => {}
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
        Err(e) => panic!("{e}"),
    };
    println!("cargo:rerun-if-changed=./blocks");
    for repo in fs::read_dir(BLOCKS_PATH).unwrap() {
        let Ok(repo) = repo else {
            continue;
        };
        let Ok(read_repo) = fs::read_dir(repo.path()) else {
            continue;
        };
        let mut generated_blocks = String::new();
        for file in read_repo {
            // some weird try block hack
            let Some(file) = || -> Option<_> {
                let file = file.ok()?;
                let file_type = file.file_type().ok()?;
                if file_type.is_dir() {
                    return None;
                }
                let file_path = file.path();
                let file_extension = file_path.extension()?.to_str()?;
                if file_extension == "yml" || file_extension == "yaml" {
                    Some(file)
                } else {
                    None
                }
            }() else {
                continue;
            };

            let Ok(file_content) = fs::read(file.path()) else {
                continue;
            };

            let Ok(blocks) = sb_block_def_genie::from_slice(&file_content) else {
                continue;
            };

            let result = block_definer::define(
                &blocks,
                &block_definer::Paths {
                    block_field_builder: "Bfb".to_string(),
                    block_input_builder: "Bib".to_string(),
                    block_normal_builder: "BlockNormalBuilder".to_string(),
                    stack_builder: "StackBuilder".to_string(),
                },
            );
            generated_blocks.push_str(&result.0);
        }
        let generate_to_path = out_dir.as_ref().join(repo.path());
        println!("{}", generate_to_path.display());
        fs::write(generate_to_path, &generated_blocks).unwrap();
    }
}
