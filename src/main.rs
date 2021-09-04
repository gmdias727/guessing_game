use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Adivinhe o numero!");
    
    let magic_number = rand::thread_rng().gen_range(1..101);

    // println!("O numero secreto é: {}", secret_number);

    loop {

        println!("Insira aqui seu numero.");
        
        let mut guess = String::new();
        
        // To summarize, the 
        // let mut guess = String::new(); line has created
        // a mutable variable that is currently bound to a new, empty instance of a String. Whew!
        
        io::stdin()
        .read_line(&mut guess)
        .expect("Falhou em ler linha");
        
        let guess: u32 = guess
        .trim()
        .parse()
        .expect("Por favor insira um número");
        
        println!("Você chutou o numero: {}", guess);
        
        match guess.cmp(&=magic_number) {
            Ordering::Less => println!("Está muito abaixo!"),
            Ordering::Greater => println!("Está muito acima!"),
            Ordering::Equal => {
                println!("Ganhou!");
                
                break;
            }
        }
    }

        
}
