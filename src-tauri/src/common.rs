use md5::{Digest, Md5};
use std::fmt::Write;
use std::io::Read;

/// compress a file with bzip2
/// # example
/// ```
/// let data=cmpr_bzip2_file("test.txt");
/// std::fs::write("test.bz", data).unwrap();
/// ```
pub fn cmpr_bzip2_file(filename: &str) -> Vec<u8> {
    let f = std::fs::File::open(filename).unwrap();
    let mut compressor = bzip2::read::BzEncoder::new(f, bzip2::Compression::best());
    let mut buf = Vec::new();
    compressor.read_to_end(&mut buf).unwrap();
    buf
}

/// decompress a file with bzip2
/// # example
/// ```
/// let v=decmpr_bzip2_file("test.bz");
/// let s=String::from_utf8(v).unwrap();
/// println!("{}",s);
/// ```
pub fn decmpr_bzip2_file(filename: &str) -> Vec<u8> {
    let f = std::fs::File::open(filename).unwrap();
    let mut compressor = bzip2::read::BzDecoder::new(f);
    let mut buf = Vec::new();
    compressor.read_to_end(&mut buf).unwrap();
    buf
}

/// get file name of a path
pub fn file_name(path: &str) -> String {
    let p = std::path::Path::new(path);
    p.file_name().unwrap().to_str().unwrap().to_owned()
}

/// get md5 of a file
/// # example
/// ```
/// println!("{}",md5_file("test.txt").unwrap());
/// ```
pub fn md5_file(file: &str) -> Option<String> {
    let v_data = std::fs::read(file);
    match v_data {
        Ok(d) => {
            // create a Md5 hasher instance
            let mut hasher = Md5::new();
            // process input message
            hasher.update(d);
            // acquire hash digest in the form of GenericArray,
            // which in this case is equivalent to [u8; 16]
            let result = hasher.finalize();
            let mut output = String::new();
            for byte in &result {
                write!(&mut output, "{:02X}", byte).unwrap();
            }
            Some(output)
        }
        Err(_) => None,
    }
}

/// get config directory path,if not exist,create it
/// # example
/// ```
/// let p=config_dir("test_config");
/// println!("{:?}",p);
/// //out: "C:\\Users\\username\\AppData\\Roaming\\test_config"
/// ```
pub fn config_dir(dir: &str) -> std::path::PathBuf {
    let s = dirs::config_dir().unwrap();
    let p = s.as_path().join(dir);
    //确保该配置文件夹存在
    ensure_dir(p.as_path().to_str().unwrap());
    p
}

/// ensure a dir exist,if not exist,create it!
/// # example
/// ```
/// assert_eq!(true,ensure_dir("D:\\b\\c\\d"));
/// ```
pub fn ensure_dir(dir: &str) -> bool {
    let p = std::path::Path::new(dir);
    if !p.exists() || !p.is_dir() {
        if let Err(_) = std::fs::create_dir_all(p) {
            return false;
        }
    }
    true
}

/// judge whether a file is exists
pub fn exist_file(file: &str) -> bool {
    let file = std::fs::metadata(file);
    if file.is_err() || !file.unwrap().is_file() {
        return false;
    }
    true
}
