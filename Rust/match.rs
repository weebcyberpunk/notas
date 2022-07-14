use std::cmp::Ordering;

fn main() {

    // Matches lembrar switches e podem ser usados como, porém, é
    // mais correto dizer assim:
    //
    // Matches pegam um pattern e veêm qual deles corresponde.
    //
    // Normalmente se usa com enums, como no exemplo:

    // comparar com cmp() uma Ordering para ver se o número é igual 
    // a outro:
    match num.cmp(&other_num) {
        Ordering::Less => println!("Menor!"),
        Ordering::Greater => println!("Maior!"),
        Ordering::Equal => {
            println!("Igual!");
            println!("Você venceu!");
        }
    }
    // O compilador garante que todas as possibilidades estão
    // cobertas pelo seu match.
}

// Note também que as expressões que match valida podem retornar
// qualquer tipo (diferente de if, que tem que ser bool). Aqui
// no caso são variações de um enum:
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
    // Ainda é possível pegar o valor que um enum carrega pelo match.
    // Seguindo no exemplo, entre 1999 e 2008 os quarters dos Estados
    // Unidos vinham diferente de acordo com seu estado. Outras 
    // moedas eram todas iguais, só os quarters podiam mudar. Assim, 
    // podemos encontrar o valor de um quarter de qualquer jeito
    // (caso Coin::Quarter guarde algo claro):
    fn value_and_state(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("{}", state);
                25,
            }
        }
    }
}

// Para aplicar um default em matches, é só pegar o valor carregado:
fn main() {
    let dice = player.dice();
    match dice {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => player.move(other),
        // perceba que adicionar algo depois disso seria código
        // impossível de rodar.
    }
}
// Se quiser NÃO usar o valor carregado, é só usar _:
fn main() {
    let dice = player.dice();
    match dice {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => player.rerun(), // assim o compilador não nos incomoda
    }                        // com uma váriavel não utilizada.
}

// E se quiser literalmente ignorar valores, é só usar o unit type,
// a tupla vazia:
fn main() {
    let dice = player.dice();
    match dice {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}
