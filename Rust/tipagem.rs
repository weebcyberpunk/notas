fn main() {

    // Rust tem tipagem forte, porém também tem inferência de tipos.
    //
    // O parse é simples. Ex:

    let num: u32 = string.parse();
    // Isso parseará a variável string para um u32 (num). Perceba que parse() retorna um Result que
    // precisa de handle.
    //
    // parse() pode ser invocado de qualquer coisa que possa ser parseada.

    // O compilador precisa saber de todos os tipos. Ele infere alguns, porém caso haja tipos que
    // são "impossíveis" de inferir, a tipagem explícita é necessária:
    let num: u32 = "42".parse().expect("Not a number!"); // certo, tipagem explícita
    let num = "42".parse().expect("Not a number!"); // errado, o compilador não sabe qual tipo num deve ser

    //
    // Tipos escalares representam um único valor:
    //

    // Inteiro: 
    // - Representa um número inteiro.
    // - Declarado como 'u' ou 'i' (unsigned ou signed) seguido do número de bits:
    //
    //     u8     i8
    //     u16    i16
    //     u32    i32
    //     u64    i64
    //     u128   i128
    //     usize  isize    padrão da arquitetura
    //
    // - Se não for específicado, irá utilizar como padrão o i32
    //
    // Inteiros podem ser representados com sufixo de tipo (ex: 58u8 para criar um u8 com o valor
    // de 58) e _ como separador visual:
    let x = 98_222;      // decimal
    let x = 0xff;        // hexadecimal
    let x = 0o77;        // octal
    let x = 0b1111_0000; // binário
    let x = b'A';        // byte (somente para u8)
    // Quando Rust roda no modo debug, overflow de inteiro causa panic. Quando compilado com
    // --release, o overflow causa "two's complement wrapping".

    // Número de Ponto Flutuante:
    //
    // Rust possui dois tipos flutuantes: f32 e f64. Por padrão, f64. Representados no padrão
    // IEEE-754. No código, não precisa de marcação:
    let x = 3.14;
    let x: f32 = 2.71;

    // Booleano:
    //
    // Obviamente, podem ser true ou false
    let has_runned: bool = true;
    let has_runned = false;

    // Caractere:
    //
    // O tipo mais primitivo de caráctere em Rust, o char, tem 4 bytes e representa carácteres em
    // Unicode Scalar Value.
    let c = ''; // use uma Nerd Font pra ver isso!

    //
    // Tipos compostos agrupam valores:
    //

    // Tuplas:
    // 
    // Agrupam valores não necessariamente do mesmo tipo. São criadas com tamanho fixo e cada
    // posição tem um tipo que não muda:
    let tuple: (i32, f64, u8) = (42, 54.3, 2);
    // É possível desestruturar tuplas:
    let (x, y, z) = tuple; // x é 42, y é 54.3, z é 2
    // Para acessar valores, .:
    let x = tuple.0;
    let y = tuple.1;
    // Uma tupla sem nada tem um tipo especial chamado de unit type e um valor especial chamado de
    // unit value, escrito (). Expressões implicitamente retornam-o se não retornam nada.

    // Arrays:
    //
    // Padrões. Tamanho fixo e todos os itens de mesmo tipo. Útil para forçar a alocação para a
    // stack e não heap.
    let array = [1, 2, 3];           // padrão.
    let array: [i32; 3] = [1, 2, 3]; // declarando tipo e tamanho.
    let array = [3; 5];              // declarando o valor inicial de tudo (3) e o tamanho (5).
    // acesso de elementos:
    let x = array[0]; 
    // Caso tente-se acessar elementos fora do array, Rust percebe antes de acessar memória
    // inválida. O programa é crashado por si, e não pelo OS.
}
