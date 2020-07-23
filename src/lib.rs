extern crate getopts;
use getopts::Options;

pub mod hospital{    

    use std::str;
    use std::error::Error;    

    use std::io::{ BufReader };
    use std::io::prelude::*;
    use std::fs::File;
    use std::fs::OpenOptions;

    extern crate prettytable;
    use prettytable::{Table, Row, Cell};
    use prettytable::{Attr, color};

    pub struct Comando {    
        pub fichero: String,
        indice: u8
    }
    impl  Comando {        
        pub fn nuevo(args: &Vec<String>) -> Self {
                                                
            let new_comando = match args[2].as_str() {
                "-c" => {
                    Comando{
                        fichero : "info_consultorio.txt".to_string(),
                        indice: 0
                    }
                },
                "-h" => { 
                    let fichero = Comando::get_nombre_fichero(&args[3]);                            
                    Comando{
                        fichero,
                        indice: 0
                    }                 
                },
                "-r" => {             
                    let fichero = Comando::get_nombre_fichero(&args[3]);  
                    let indice = Comando::get_indice(&args[4], &args[5]);                   
                    Comando{
                            fichero,
                            indice
                    }              
                },
                _ =>  {
                    println!("No existe esa opcion!!\n");
                    std::process::exit(1);
                }             
            };                              
            new_comando            
        }
        pub fn generar_tabla(&self) -> Result<(), Box<dyn Error>> {

            //BUFFER DEL FICHERO MANDADO
            let  contenido = File::open(&self.fichero)?;
            let mut buf = BufReader::new(&contenido);
            let  buf = buf.fill_buf()?;
            let mut line = buf.lines().map(|l| { l.unwrap() }); 
            //CONSTRUCTOR DE LA TABLA DINAMICA
            let mut table = Table::new();
        
            let titulo = match line.next(){
                Some(title) => title,
                None =>  String::from("Sin Titulo"),
            };
            let row_info = match line.next(){
                Some(col) => col,
                None =>  String::from("Sin Row Info"),
            };
            let row_info: Vec<&str> = row_info.split(',').collect();    
            let max_col = row_info.len();
        
            let row_active = match line.next(){
                Some(act) => act,
                None => String::from("Sin Row Active"),
            };
            let row_active: Vec<&str> = row_active.split(",").collect();
        
            //CREACION DE TITULO Y ROW INFO
            table.set_titles(Row::new(vec![
                Cell::new(&titulo)
                    .with_hspan(max_col)
                    .with_style(Attr::ForegroundColor(color::CYAN)),            
            ]));
            let mut vec_info_row: Vec<Cell> = Vec::new();
            for col_info in row_info {
                let new_col = Cell::new(&col_info)
                            .with_style(Attr::ForegroundColor(color::BLUE));        
                vec_info_row.push(new_col);
            }
            table.add_row(Row::new(vec_info_row));
                 
            //CREACION DEL CUERPO DE LA TABLA
            let mut contador_fin = 0;   
        
            for item in line {
                let mut vec_info_body_row: Vec<Cell> = Vec::new();
                let row_body: Vec<&str> = item.split(',').collect();
                for col_body in row_body {
                        if row_active[contador_fin] == "0" {            
                            let new_col = Cell::new(&col_body)
                                .with_style(Attr::ForegroundColor(color::WHITE));        
                            vec_info_body_row.push(new_col);
                        }
                        else{      
                            let new_col = Cell::new(&col_body)
                                .with_style(Attr::ForegroundColor(color::RED));        
                            vec_info_body_row.push(new_col);
                        }            
                    contador_fin += 1;    
                }   
                table.add_row(Row::new(vec_info_body_row));     
            }           
            table.printstd();
            Ok(())
        }
        pub fn reservar_cita(&self) -> Result<(), Box<dyn Error>>{

            //BUFFER DEL FICHERO MANDADO
            let contenido = OpenOptions::new()
                              .read(true)                      
                              .open(&self.fichero)?;
                                                    
            let mut buf = BufReader::new(&contenido);
            let  buf = buf.fill_buf()?;    
            let line = buf.lines().into_iter().map(|l| {                        
                                    l.unwrap()                            
                            });
            //BUFFER DEL FICHERO MANDADO
            let mut otro_contenido = OpenOptions::new()                        
                                .write(true)
                                .open(&self.fichero)?;
                                
            let mut row_count = 0;
            for item in line {  
                        
                    if row_count == 2 {                
                        let indice = self.indice;
                        let nuevo = Comando::confirmar_cita(&item,indice);
                        let nuevo_parrafo = format!("{}\n",nuevo);
                        otro_contenido.write_all(&nuevo_parrafo.into_bytes())?;
                    }
                    else{  
                        if row_count != 8{
                            let nuevo_parrafo = format!("{}\n",&item);            
                            otro_contenido.write_all(&nuevo_parrafo.into_bytes())?;
                        }
                        else{
                            let nuevo_parrafo = format!("{}",&item);            
                            otro_contenido.write_all(&nuevo_parrafo.into_bytes())?;
                        }
                    }            
                row_count += 1;                    
            }
            Ok(())
        }    
        fn confirmar_cita(v: &String, indice: u8) -> String {

            let info_cupo = v.clone();    
            let mut bytes = info_cupo.into_bytes();        
            let refbytes  = bytes.as_mut_slice();
            let mut condator_indice: u8 = 1;
        
            for item in 0..refbytes.len() {
                //println!("{}",item);
                if refbytes[item] != 44 {
                    if indice == condator_indice{   
                        
                        if refbytes[item] == 49 {                   
                            println!("YA ESTA OCUPADO ESE HORARIO.\n\n");
                            std::process::exit(1);
                        }
                        else{
                            refbytes[item] = 49;
                        }                
                    }
                    condator_indice += 1;
                }      
            }    
            let s = match String::from_utf8(bytes){
                Ok(l) => l,
                Err(e) =>panic!("{}",e)
            };    
            s            
        }
        fn get_indice(_dia: &String, _hora: &String) -> u8 {
            
            let mut indice: u8 = 0;
            indice += match _dia.as_str() {
                "lunes" => 0,
                "martes" => 1,
                "miercoles" => 2,
                "jueves" => 3,
                "viernes" => 4,
                "sabado" => 5,
                _ => {
                    println!("{} No es un argumento valido\n", _dia);
                    std::process::exit(1);
                }
            };
            indice += match _hora.as_str() {
                "8-9" => 1,
                "9-10" => 7,
                "10-11" => 13,
                "11-12" => 19,
                "12-1" => 25,
                _ => {
                    println!("{} No es un argumento valido\n", _hora);
                    std::process::exit(1);
                }
            };           
            indice
        }   
        fn get_nombre_fichero(_id: &String) -> String {
                
            let nombre_fichero = match _id.as_str() {
                "1" | "2" | "3" | "4" | "5" => format!("consultorio{}.txt", _id),                                    
                _ =>  {
                    println!("No existe ese consultorio!!\n");
                    std::process::exit(1);
                }                                                   
            };           
            nombre_fichero
        }        
    }
}
pub fn inicia_opciones() -> Options {    
    let mut opts = Options::new();
    opts.optflag("", "ayuda", "lista de opciones disponibles.");
    opts.optflag("c", "consultorio", "lista de consultorios.");
    opts.optopt("h", "horario", "horarios de consultorio","n°");
    opts.optopt("r", "", "reserve una cita ejemplo: 1 lunes 8-9", "n° dia hora");
    opts    
}
pub fn mensaje_opciones(opts: &Options) {
    println!("{}",opts.usage("Aplicacion: Cita de Hospital"));
}