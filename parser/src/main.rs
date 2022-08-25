extern crate walkdir;
use std::env;
use std::fs;
use walkdir::WalkDir;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 1 {
        panic!("Please provide the program id as an argument. Usage: ./X <program_id>")
    }
    let project_dir = format!(
        "/home/wj/projects/solana-tx-debug/programs/{}/code",
        args[1]
    );

    let mut dirs = vec![];
    let mut files = vec![];
    let mut files_memfs = vec![];

    for e in WalkDir::new(project_dir.clone())
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            files.push(String::from(e.path().to_string_lossy()));
            files_memfs.push(String::from(
                e.path()
                    .strip_prefix(project_dir.clone())
                    .unwrap()
                    .to_string_lossy(),
            ));
        } else {
            if e.path().to_string_lossy() != project_dir {
                dirs.push(String::from(
                    e.path()
                        .strip_prefix(project_dir.clone())
                        .unwrap()
                        .to_string_lossy(),
                ));
            }
        }
    }

    let mut ts_string = format!(
        r#"export const dirNames = {:?};
export const fileNames = {:?};
    "#,
        dirs, files_memfs
    );

    for (i, name) in files.iter().enumerate() {
        let data = fs::read_to_string(&name).expect("File not found");
        ts_string = format!(
            r#"{}
export const {} = `{}`"#,
            ts_string,
            format!("_{}", i),
            data
        );
    }
    println!("{}", ts_string);
    fs::write("projectFiles.ts", ts_string).expect("Unabel to write file");
}
