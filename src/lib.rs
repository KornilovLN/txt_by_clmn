use std::io;
use std::io::Write;
use std::error::Error;

extern crate ansi_term;
use ansi_term::Colour;

mod md_utils;
mod md_about;
mod md_concept;
mod md_tocolumn;

//================================================================================================

pub struct Params {
	pub paths: Vec<String>,
}

impl Params {
    pub fn new(pat: &Vec<String>) -> Result<Params, &'static str> {
		Ok(Params {paths: pat.to_vec()})
	}
}

//================================================================================================

//--- подготовка к осн циклу работы
fn prepare_run() {
    println!();
    md_about::target("txt_by_clmn",
                     "Инструментальная программа сведения текстов по колонкам");

    println!();
    md_about::get_json_from_file();

    println!();
    concept_info();

    md_utils::iron();

    md_utils::waiter(5);    
}

//--- Функция вывода файла с подробным комментарием к программе
fn concept_info() {
    let reading_file_result: Result<(), io::Error> = md_concept::read_concept();
    let _file_result = match reading_file_result {
        Ok(file) => file,
        Err(error) => {
            let frm_err = format!("{}: {}",
                                Colour::Red.paint("Проблема с 'concept.txt'".to_string()),
                                Colour::Yellow.paint(error.to_string())
            );
            panic!("{}",frm_err);
        },
    };
}

//--- Чтение числа из ст. ввода
//--- https://gist.github.com/kudrevatykh/32b33c642998fea945a8
fn read_i32(comment:&str) -> i32 {
    print!("{}", comment);
    io::stdout(). flush();

    let mut string: String = String::new();

    io::stdin().read_line(&mut string)
        .ok()
        .expect("Error read line!");

    return string.trim().parse::<i32>().unwrap();
}

//--- Функция ввода варианта с клавиатуры
fn prompt_select_work() -> i32 {
    println!();
    let frmfn = format!("{} ",
                            Colour::Yellow
                                .bold()
                                .paint("Выбор нужной работы"));
    println!("{}\n", frmfn);

    println!("1  =>\tmd_tocolumn::readfiles('path');");
    println!("2  =>\tmd_tocolumn::go();");
    println!();
    println!("10 =>\tИнформация о программе из concept.txt");
    println!("11 =>\tКраткая информация о программе");
    println!("12 =>\tОб авторе");

    println!("\n0  =>\tКонец работы");
    println!();

    let frmprompt = format!("{} ", 
        Colour::Yellow.bold().paint("Выберите номер варианта:"));

    read_i32(&frmprompt)
}

//=== Секция Run =================================================================================

pub fn run(params: Params) -> Result<(), Box<dyn Error>> {
    //--- структура для считывания about.json
    let _about = md_about::StAbout::new("", "", "", "",
                                                    "", "", "");
    prepare_run();
   
    //--- Остальной код --------------------------------------------------------------------------

    loop {

        match prompt_select_work() {    
            1 => { md_utils::title_with_border("md_tocolumn: readfiles(paths: &Vec<String>)");
                   md_tocolumn::readfiles(&params.paths, true);
            },
            2 => { md_utils::title_with_border("md_tocolumn: Распечатка файлов по колонкам");
                   md_tocolumn::go(&params.paths);
            },            


            10=> { md_utils::title_with_border("concept.txt - расширенное описание программы");
                   concept_info();
            },
            11=> { println!();
                   md_about::target("txt_by_clmn",
                                    "Инструментальная программа сведения текстов по колонкам");
            },
            12=> { println!();
                   md_about::get_json_from_file(); 
            },       


            0 => { print!("\n{}", "Конец работы");
                break;
            },

            _ => { let mess = "Ошибка: Нет такого варианта, повторите ввод"; 
                   let frmerr = format!("{} ",Colour::Red.bold().paint(mess));
                   println!("{}", frmerr);
            },
        };

        println!();
    }    

    Ok(())
}

