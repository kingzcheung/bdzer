use sha2::{Digest, Sha256};
use std::io::Read;
use std::path::PathBuf;
use std::{
    collections::HashMap,
    fs::DirEntry,
    io::{Error, Write as _},
};
#[derive(Debug)]
pub struct Bulldozer {
    base_path: Vec<PathBuf>,
    include_ext: Option<Vec<String>>,
}

impl Bulldozer {
    pub fn new(base_path: Vec<PathBuf>) -> Self {
        Bulldozer {
            base_path,
            include_ext: None,
        }
    }

    pub fn run(&self) -> Result<HashMap<String, Vec<String>>, Box<dyn std::error::Error>> {
        let mut hash_map: HashMap<String, Vec<String>> = HashMap::new();

        for base_path in &self.base_path {
            generate_hash_list(&mut hash_map, base_path, &self.include_ext)?;
        }

        let map = hash_map.into_iter().filter(|(_, v)| v.len() > 1).collect();

        Ok(map)
    }

    pub fn set_include_ext(&mut self, include_ext: Option<Vec<String>>) {
        self.include_ext = include_ext;
    }
}


/// 生成文件的哈希列表。
/// 
/// 此函数遍历指定路径下的所有文件和子目录，对每个文件计算其哈希值，并将文件路径添加到哈希值对应的列表中。
/// 如果指定了包含的文件扩展名，则只处理这些扩展名的文件。支持递归处理子目录。
/// 
/// 参数:
/// - hash_map: 参考mut的HashMap，用于存储文件哈希值和对应文件路径的列表。
/// - entity_path: PathBuf类型，表示要处理的目录路径。
/// - include_ext: Option<Vec<String>>类型，可选参数，表示要包含的文件扩展名列表。
/// 
/// 返回值:
/// - Result<(), Box<dyn std::error::Error>>: 表示操作是否成功，错误时提供详细信息。
fn generate_hash_list(
    hash_map: &mut HashMap<String, Vec<String>>,
    entity_path: &PathBuf,
    include_ext: &Option<Vec<String>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let entities = std::fs::read_dir(entity_path)?;
    let stdout = std::io::stdout();
    let mut handle = stdout.lock(); // 获取stdout的独占访问权

    let entities: Vec<Result<DirEntry, Error>> = entities.into_iter().collect();

    'e: for entity in entities {
        let entity = entity?;
        let entity_path = entity.path();
        if entity_path.is_file() {
            let mut inclu_tag = false;
            if let Some(inclu_ext) = include_ext {
                if let Some(ext) = entity_path.extension() {
                    'inclu: for e in inclu_ext {
                        if ext.eq_ignore_ascii_case(e) {
                            inclu_tag = true;
                            break 'inclu;
                        }
                    }
                }
            }
            if !inclu_tag {
                continue 'e;
            }

            let hash_str = hash_file(&entity_path)?;
            let file_path_str = entity_path.to_str().unwrap_or_default();
            // println!("正在处理: {}",file_path_str);
            write!(
                handle,
                "\x1b[1K\r正在处理: {}",
                entity_path
                    .file_name()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default()
            )
            .unwrap(); // \r 将光标移到行首
            handle.flush().unwrap(); // 确保输出立即显示
            hash_map
                .entry(hash_str)
                .or_default()
                .push(file_path_str.to_string());
        } else {
            generate_hash_list(hash_map, &entity_path, include_ext)?;
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
mod tests {
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
