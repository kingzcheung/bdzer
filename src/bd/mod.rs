use std::{collections::HashMap, fs::DirEntry, io::{Error, Write as _}};
use std::io::Read;
use std::path::PathBuf;
use sha2::{Digest, Sha256};
#[derive(Debug)]
pub struct Bulldozer {
    base_path: Vec<PathBuf>,
}

impl Bulldozer {
    pub fn new(base_path: Vec<PathBuf>) -> Self {
        Bulldozer {
            base_path
        }
    }

    pub fn run(&self) -> Result<HashMap<String, Vec<String>>, Box<dyn std::error::Error>> {
        let mut hash_map: HashMap<String, Vec<String>> = HashMap::new();


        for base_path in &self.base_path {
            generate_hash_list(&mut hash_map, base_path)?;
        }


        let map = hash_map.into_iter().filter(|(_, v)| v.len() > 1).collect();

        Ok(map)
    }
}


fn generate_hash_list(hash_map: &mut HashMap<String, Vec<String>>,entity_path:&PathBuf) -> Result<(),Box<dyn std::error::Error>> {
    let entities = std::fs::read_dir(entity_path)?;
    let stdout = std::io::stdout();
    let mut handle = stdout.lock(); // 获取stdout的独占访问权

    let entities:Vec<Result<DirEntry,Error>> = entities.into_iter().collect();


    for entity in entities {
        let entity = entity?;
        let entity_path = entity.path();
        if entity_path.is_file() {
            let hash_str = hash_file(&entity_path)?;
            let file_path_str = entity_path.to_str().unwrap_or_default();
            // println!("正在处理: {}",file_path_str);
            write!(handle, "\x1b[1K\r正在处理: {}", entity_path.file_name().unwrap_or_default().to_str().unwrap_or_default()).unwrap(); // \r 将光标移到行首
            handle.flush().unwrap(); // 确保输出立即显示
            hash_map.entry(hash_str).or_default().push(file_path_str.to_string());
        } else{
            generate_hash_list(hash_map, &entity_path)?;
        }
        
    }

    Ok(())
}


/// 对文件内容进行hash
fn hash_file(path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let mut hasher = Sha256::new();
    let mut file = std::fs::File::open(path)?;
    const BUFFER_SIZE: usize = 64 * 1024; // 64KB
    let mut buffer = vec![0; BUFFER_SIZE];
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

#[cfg(test)]
mod tests
{
    use crate::bd::hash_file;

    #[test]
    fn test_hash_file() {
        let mut p = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("test_data/test1.txt");

        let res = hash_file(&p);
        dbg!(&res);
        assert!(res.is_ok())
    }

    #[test]
    fn test_run() {
        let mut p = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("test_data");

        let res = super::Bulldozer::new(vec![p]).run();
        dbg!(&res);
        assert!(res.is_ok())
    }
}
