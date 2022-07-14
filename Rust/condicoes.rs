fn main() {

    // A principal condicional é o if:
    // (na verdade o match geralmente é mais útil mas veja isso
    // nas notas só dele).
    let num = 3;
    if num < 5 {
        println!("Menor que 5!");

    } else if num == 5 {
        println!("Igual a 5!");

    } else {
        println!("Maior que 5!");

    }

    // IMPORTANTE!
    // Diferente de C, em Rust as condições devem ser do tipo bool.

    // Já que if é uma expressão, pode ser colocada num let:
    let condition = true;
    let num = if condition {
        1 
    } else { 
        0 
    };
    // Perceba que tanto if quanto else devem ser do mesmo tipo para 
    // que o compilador saiba o tipo em compile-time (um dos 
    // mecanismos de segurança de Rust).

    // Um açúcar que pode ser feito com if é o if let. Quando se tem
    // um match que só quer rodar algo em uma única opção e ignorar
    // o resto, (ou rodar um default) pode substituir por algo assim:
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("maximum: {}", max);

    } else {
        println!("nesse caso seria só None mesmo");
    }
}
