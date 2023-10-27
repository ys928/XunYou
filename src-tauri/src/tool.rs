use crate::common;

#[tauri::command]
pub fn open_novel_txt(filename:&str) ->Vec<String>{
    let str=std::fs::read_to_string(filename).unwrap_or_else(|_e|{
        panic!("error");
    });
    let str=str.replace("\r", "");
    let v:Vec<&str>=str.split("\n").collect();
    let mut ret=Vec::new();
    for i in v{
        ret.push(i.to_string());
    }
    ret
}

#[tauri::command]
pub fn open_novel(filename:&str) ->Vec<String>{
    let v=common::decmpr_bzip2_file(filename);
    let s=String::from_utf8(v).unwrap_or_else(|_e|{
        panic!("error");
    });
    let s=s.replace("\r", "");
    let v:Vec<&str>=s.split("\n").collect();
    let mut ret=Vec::new();
    for i in v{
        ret.push(i.to_string());
    }
    ret
}
#[tauri::command]
pub fn txt_to_bzip(txt:&str){
    let v=common::cmpr_bzip2_file(&txt);
    let mut name=common::file_name(&txt);
    let p=std::path::Path::new(&txt);
    let p=p.parent().unwrap();
    name.drain(name.find('.').unwrap()..);
    name=name+".novel";
    let name=p.join(name);
    std::fs::write(name, v).unwrap();
}