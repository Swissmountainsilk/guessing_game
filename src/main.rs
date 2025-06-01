use rand::Rng;
use std::io;
use std::cmp::Ordering;
use std::process;


fn main() -> ! {
    println!("Rate die Zahl zwischen 0 und 100!");

    let secret_number: u32 = rand::thread_rng().gen_range(0..=100);
    
    println!("Die geheime Zahl ist: {secret_number}");

    loop{
        println!("Bitte gebe eine Schätzung ab");

    

        let mut guess: String = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Fehler beim Lesen der Zeile");

        //Prüfen ob die eingegebene Zahl wirklich eine Zahl ist und dann die Variabel guess in eine Zahl u32 umwandeln. Wenn nicht dann Fehler ausgeben.
        let guess: u32 = match guess.trim().parse(){

            Ok(num) => num,
            Err(_) => {
                println!("Bitte eine gültige Zahl eingeben! \n *{guess}* ist keine Zahl");
                continue;
            }
        };

        
    

        println!("Du hast geschätzt: {guess}");

        //Vergleichen der Schätzung mit der geheimen Zahl. Wenn die Richtige Zahl geraten wurde, dann wird das Programm beendet.
        match guess.cmp(&secret_number) {

            Ordering::Less => println!("Zu niedrig!"),
            Ordering::Greater => println!("Zu hoch!"),
            Ordering::Equal => {
                println!("Richtig geraten!\nDas Programm wird beendet.");
                //Programm beenden
                process::exit(0);
            },
        }
    }
        
    
}