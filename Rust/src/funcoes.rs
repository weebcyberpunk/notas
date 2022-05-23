// Funções em Rust podem ser declaradas em qualquer lugar fora de outras. Declaradas em algum
// lugar, podem ser chamadas de qualquer lugar. Não importa se antes ou depois da função que chama.
// De resto, são bem padrões na declaração:
fn main() {
    func();
}

fn func() {
    println!("Yay");
}
// Quando receberem argumentos, os tipos DEVEM ser especificados:
fn soma(x: i32, y: i32) {
    let z = x + y;
    println!("A soma de {} e {} é {}.", x, y, z);
}
// Funções implicitamente retornam o valor da última expressão (ela deve ser escrita sem ; para que
// seja uma expressão e não statement). Ou pode se usar return para retornar mais cedo. O tipo é
// necessariamente identificado com ->:
fn answer() -> i32 {
    42
}
fn another_function() {
    let answer = answer(); // answer vale 42
}
