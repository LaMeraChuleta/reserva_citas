use reserva_citas:: { inicia_opciones, mensaje_opciones };
use reserva_citas::hospital::Comando;
use std::env;

fn main() {
    
    let  args: Vec<String> = env::args().collect();

    let opts = inicia_opciones();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) =>  m,
        Err(_) => {            
            mensaje_opciones(&opts);
            println!("Opcion Invalida!!\n");            
            std::process::exit(1);
        }
    };  
    if matches.opt_present("ayuda") {                  
        mensaje_opciones(&opts);
    } 
    else {
        let _comando =  Comando::nuevo(&args);
        
        if matches.opt_present("c") && _comando.generar_tabla().is_err() {
                println!("Ocurrio un Error :(");
                std::process::exit(1);

        }
        if matches.opt_present("r") && _comando.reservar_cita().is_err() {
                println!("Ocurrio un Error :(");
                std::process::exit(1);
        }    
        if matches.opt_present("h") && _comando.generar_tabla().is_err() {               
      
            println!("Ocurrio un Error :(");
            std::process::exit(1);
           
        }
    }
}
