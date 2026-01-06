use rand::Rng;
use std::io;
use std::cmp::Ordering;
fn main() {

    println!("Escolhan um numero!");

    let secret_numeber: u32 = rand::rng().random_range(1..=100);

    println!("O numero secreto e:{}", secret_numeber);

    loop {
        println!("Digite um numero:");
        let mut guees: String = String::new();

        io::stdin()
            .read_line(&mut guees)
            .expect("Falha ao ler a linha");

        let guees: u32 = guees.trim().parse().expect("Digite um numero valido!");
        println!("Voce escolheu: {}", guees);

        match guees.cmp(&secret_numeber){
            Ordering::Less => println!("E menor!!!"),
            Ordering::Greater => println!("e maior!!!"),
            Ordering::Equal => {
                println!("Voce acertou !!");
                break;
            }

        }

    }


}


