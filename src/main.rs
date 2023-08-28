use std::{fs::{self, DirEntry}, path::Path, env};

fn get_filename_as_str(path: &Path) -> Option<&str> {
    if let Some(filename) = path.file_name() {
        filename.to_str()
    } else {
        None
    }
}

fn list_dir(path: &DirEntry, level: u8, idx: i32) {
    
    let folder:String = ["  ".repeat((idx).try_into().unwrap()),"\u{f07c}".try_into().unwrap()].join("");
    let hyphen_tab:String = "  ".repeat((idx).try_into().unwrap());
    let pipe_tab:String = "  ".repeat((idx + 1).try_into().unwrap());


    if level > 0 {
        if let Some(fname) = get_filename_as_str(&path.path()){
            if path.path().is_dir() {
                println!("{}-{}",folder,fname);    
            }else{
                println!("{}+-{}",hyphen_tab,fname);    
            }
            
        }
        if path.path().is_dir() {
            
            if let Ok(entries) = fs::read_dir(path.path()) {
                if entries.count() > 0{
                    println!("{}|",pipe_tab);
                }
            }

            let paths = fs::read_dir(path.path()).unwrap();
            
            for sub_path in paths {
                list_dir(&sub_path.unwrap(), level - 1, idx + 1);
            }
        } 
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut search_pathname = "./";
    let level=3;
    if args.len() > 1{
        search_pathname = &args[1];
        println!("Searching path {}",search_pathname);
    }
    
    let paths = fs::read_dir(search_pathname).unwrap();
    
    println!("{}",search_pathname);
    for path in paths {
        list_dir(&path.unwrap(), level, 1);
    }
}
