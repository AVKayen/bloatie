use std::env;
use std::path::Path;
use std::fs::{File, remove_file};
use std::io::Result;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 4  {
        let pathstr = &args[2];
            let name = &args[3];
            let path = Path::new(pathstr);
            if path.is_dir() {
                if args[1] == "bloat" {
                    bloat_folder(path, &name)?;
                } else if args[1] == "debloat" {
                    debloat_folder(path, &name)?;
                } else {
                    eprintln!("Nieprawidłowa metoda: {}. Poprawne to 'bloat' i 'debloat'.", args[1]);
                }
            
        } 
            Ok(())
        }
     else {
        println!("Użycie:");
        println!("  $ bloatie bloat . hehe.bloater     # tworzy plik \"haha.bloater\" w każdej podścieżce ścieżki \".\"");
        println!("  $ bloatie bloat . hehe.bloater     # usuwa plik \"haha.bloater\" z każdej podścieżki ścieżki \".\"");
        Ok(())
    }
}

fn bloat_folder(path: &Path, name: &str) -> Result<()> {
    println!("Tworzenie {} w {:?}", name, path);
    let file = Path::new(path).join(name);
    File::create(file)?;
    for entry in path.read_dir()? {
        let dir = entry?.path();
        if dir.is_dir() {
            bloat_folder(dir.as_path(), name)?;
        }
    }
    Ok(())
}

fn debloat_folder(path: &Path, name: &str) -> Result<()> {
    println!("Usuwanie {} z {:?}", name, path);
    let file = Path::new(path).join(name);
    remove_file(file)?;
    for entry in path.read_dir()? {
        let dir = entry?.path();
        if dir.is_dir() {
            debloat_folder(dir.as_path(), name)?;
        }
    }
    Ok(())
}