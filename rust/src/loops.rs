fn main() {

    // Para fazer um loop infinito, loop com break e continue padrões:
    loop {

        let condition = true;

        if condition {
            break;

        } else {
            continue;

        }
    }

    // Para o caso de termos loops dentro de loops, Rust possibilita labels de loops, para
    // especificar de qual loop devemos sair ou continuar:

    'loop1 loop {
        'loop2 loop {
            let condition = true;
            
            if condition {
                break 'loop1;

            } else {
                continue 'loop1;

            }
        }
    }

    // loops também são expressões e podem ser usados em let:
    let mut num = 0;

    let result = loop {

        num += 1;
        if num == 5 {
            break num;
        }
    }

    // while também é suportado em Rust
    let mut num = 3;
    while num != 0 {
        println!("{}!", num);
        num -= 1;
    }

    // O for de Rust é feito com elementos de coleções, bem parecido com o for de Python
    let names [ "GG", "Alice", "Bob" ];
    for name in names {
        println!("Hello, {}!", name);

    }

    // O Range da std é uma coleção:
    for i in (1..10) {
        println!("{}", i);
    }

    // um método útil para ele nesse caso é o rev():
    for i in (1..10).rev() {
        println!("{}", i);
    }
}
