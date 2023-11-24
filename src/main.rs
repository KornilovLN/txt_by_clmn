use std::env;

use ansi_term::Colour;
use std::process;

use txt_by_clmn::Params; 

fn main() {
    //--- аргументы - имена текстовых файлов через пробел (не более 4-х)
    let args: Vec<String> = env::args().collect();

    //--- парсер аргументов и загрузка ими структуры params
    let params = Params::new(&parse_args(&args))
        .unwrap_or_else(|err| {
		eprintln!("{} {}",Colour::Red.paint("Проблема с аргументами: "), err);
		process::exit(1);
	});

    //--- ошибки в функции run() придут сюда
    if let Err(e) = txt_by_clmn::run(params) {
        let mess = "Main: Ошибка в приложении".to_string();
        let frm_err = format!("{}",Colour::Red.paint(mess));
        eprintln!("{}: {}", frm_err, e);
        process::exit(1);
    }
}

//--- Парсер командной строки
fn parse_args(args: &[String]) -> Vec<String> {
    let mut pat: Vec<String> = vec![];
    let cnt = args.len();
    if cnt > 1 {
        for i in 1..cnt {
            let el = args[i].trim().to_owned();
            pat.push(el);    
        }
    }
    
    pat
}
