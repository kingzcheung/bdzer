use clap::Parser;
use std::{collections::HashMap, path::PathBuf, vec};
pub mod bd;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// path to the base directory
    pub base_path: Vec<PathBuf>,

    /// include file extension
    #[arg(short, long, default_value = "rs")]
    pub include_ext: Option<Vec<String>>,

    #[arg(short, long)]
    pub yes: bool,
}

pub fn cli_run(
    base_path: Vec<PathBuf>,
    include_ext: Option<Vec<String>>
) -> Result<HashMap<String, Vec<String>>, Box<dyn std::error::Error>> {
    let mut bd = bd::Bulldozer::new(base_path);
    bd.set_include_ext(include_ext);
    bd.run()
}

#[allow(clippy::for_kv_map)]
pub fn only_one_for_key(map: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut files = vec![];

    for (_, v) in map {
        if v.len() > 1 {
            files.extend(v[0..v.len() - 1].to_vec())
        }
    }

    files
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn test_cli_run() {
        let base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let res = super::cli_run(vec![base_path.to_path_buf()],None);
        assert!(res.is_ok());
        let res = res.unwrap();
        for (_, v) in res {
            dbg!(v);
        }
    }

    #[test]
    fn test_only_one_for_key() {
        let mut base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        base_path.push("test_data");
        let res = super::cli_run(vec![base_path.to_path_buf()],None);
        assert!(res.is_ok());
        let res = res.unwrap();
        let res = super::only_one_for_key(&res);
        assert!(res.len() == 3);
    }
}
