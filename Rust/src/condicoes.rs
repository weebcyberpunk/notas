fn main() {

    // A principal condicional é o if
    let num = 3;
    if num < 5 {
        println!("Menor que 5!");

    } else if num == 5 {
        println!("Igual a 5!");

    } else {
        println!("Maior que 5!");

    }

    // IMPORTANTE!
    // Diferente de C, em Rust as condições devem ser do tipo bool

    // Já que if é uma expressão, pode ser colocada num let:
    let condition = true;
    let num = if condition {
        1 
    } else { 
        0 
    };
    // perceba que tanto if quanto else devem ser do mesmo tipo para que o compilador saiba o
    // tipo em compile-time (um dos mecanismos de segurança de Rust)
}
