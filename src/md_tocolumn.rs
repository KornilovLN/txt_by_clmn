use std::fs::File;
use std::io::{self, BufReader, BufRead, Error};
use std::path::Path;

use crate::md_utils;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Clone)]
pub struct FileInfo {
    vstr: Vec<String>,  //--- собственно вектор строк из файла
    vlen: Vec<usize>,   //--- длины строк в этом файле
    maxlen: usize,      //--- самая большая длина строки
    cnt: usize,         //--- число строк в файле  
}

//--- Перебираем все файлы согласно вектору путей, полученных в арг.
//https://doc.rust-lang.ru/stable/rust-by-example/std_misc/file/read_lines.html
pub fn readfiles(paths: &Vec<String>, deb: bool) -> Vec<FileInfo> {
    if deb == true { println!(); }

    let mut vec_fileinfo: Vec<FileInfo> = vec![];

    for pat in paths {

        // Читаем очередной файл, который должен существовать 
        if let Ok(lines) = read_lines(&pat) {

            let mut vstr: Vec<String> = vec![]; //--- строки из файла
            let mut vlen: Vec<usize> = vec![];  //--- длины строк 
            let mut maxlen: usize = 0;          //--- большая длина строки
            let mut cnt: usize = 0;             //--- число строк в файле

            // Получает итератор, который возвращает Option
            for line in lines {
                if let Ok(stroka) = line {

                    let stroka = stroka.trim_end().to_owned();

                    vstr.push(stroka.clone());

                    let len = stroka.chars().count();
                    vlen.push(len);

                    if maxlen < len { maxlen = len; }

                    cnt += 1;

                    if deb == true { println!("[{:3}][{:3}]\t{}", cnt, len, stroka); }
                }      
            }  

            let mut fileinfo = FileInfo {vstr: vstr,
                                                   vlen: vlen, 
                                                   maxlen: maxlen, 
                                                   cnt: cnt};
            vec_fileinfo.push(fileinfo); 

            if deb == true { println!("\n<<< maxlen: {} >>>\n", maxlen); }
        }               
    } 

    vec_fileinfo
}


pub fn go(paths: &Vec<String>) {
    //--- зачитаем файлы в свои блоки структур
    let vec_fileinfo: Vec<FileInfo> = readfiles(paths, false);

    //--- найдем максимальное к-во строк во всех блоках
    let mut maxstrok = 0_usize;
    for block in &vec_fileinfo {
        if block.cnt > maxstrok { maxstrok = block.cnt}
    }

    //--- рамка гоизонтальная         
    print!("+");
    for block in vec_fileinfo.clone() {  
        md_utils::print_chars("-", block.maxlen as i32 + 2);                
        print!("+");
    }  
    println!();

    //--- окошки для названий
    print!("|");
    for block in vec_fileinfo.clone() {  
        md_utils::print_chars(" ", block.maxlen as i32 + 2);                
        print!("|");
    }  
    println!();

    //--- рамка гоизонтальная
    print!("+");
    for block in vec_fileinfo.clone() {  
        md_utils::print_chars("-", block.maxlen as i32 + 2);                
        print!("+");
    }  
    println!();

    //--- текст в рамках вертикальных построчно
    for j in 0..maxstrok {
        print!("| ");
        for block in vec_fileinfo.clone() {  
            if j >= block.cnt { //--- печатать пробелы, если строки блока закончились
                md_utils::print_chars(" ", block.maxlen as i32);                
            }else{              //--- печатать строки блока и дополнять их пробелами
                print!("{}", block.vstr[j]);
                md_utils::print_chars(" ", block.maxlen as i32 - block.vlen[j] as i32);               
            }
            print!(" | ");
        }  
        println!("");
    }

    //--- рамка гоизонтальная
    print!("+");
    for block in vec_fileinfo.clone() {  
        md_utils::print_chars("-", block.maxlen as i32 + 2);                
        print!("+");
    }  
    println!();

}

/*
fn print_chars(ch: &str, n: i32) {
    for i in 0..n {
        print!("{}",ch);
    }
}
*/

