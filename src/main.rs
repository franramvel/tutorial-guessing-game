use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Adivina el número que va a salir, del 1 al 100");

    println!("Por favor, ingresa tu predicción");

    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    //para rangos de numeros es el siguiente formato start..=end


    loop{
        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Fallo al intentar leer");

    
        let guess: u32 = match guess.
        trim()
        .parse() {
            Ok(num)=>num,
            Err(_)=>{
                println!("El input no es adecuado, por favor ingresa solo números");
                continue
            }
        };

        if guess > 100
        {
            println!("El número no debe ser mayor a 100");
            continue;
        }
        
    
        println!("Tu predicción fue: {guess}");
        //println!("El número que salió fue: {secret_number}");
    
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Te falta!"),
            Ordering::Greater => println!("Te pasaste!"),
            Ordering::Equal => {
                println!("Ganaste!");
                break;
            },
        }
    }
    
}