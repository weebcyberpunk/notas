use std::cmp::Ordering;

fn main() {

    // Blocos de match são como switches, que pegam uma expressão e realizam código de acordo com o
    // resultado.
    //
    // Ex: comparar com cmp() uma Ordering para ver se o número é igual a outro:

    match num.cmp(&other_num) {
        Ordering::Less => println!("Menor!"),
        Ordering::Greater => println!("Maior!"),
        Ordering::Equal => {
            println!("Igual!");
            println!("Você venceu!");
        }
    }
}
