fn main() {
    // Rust é feita para ser usada em baixo nível, porém de maneira amigável, principalmente quanto
    // ao gerenciamento de memória.
    //
    // Rust provém todo um ambiente para se programar: com um gerenciador de dependências e
    // build system (Cargo), servidor lsp (rls ou o mais novo rust-analyzer) e outras
    // utilidades.
    //
    // Infelizmente o estilo de Rust é indentar com quatro espaços e não tab.
    //
    //
    // Declarações de funções utilizam a keyword fn e a função main é rodada no início do programa.
    //
    // Para chamar um macro, utiliza-se !:
    println!("Hello, World!");
    // macros são diferentes de funções, que não precisam do !.
    //
    // Para definir váriaveis, let:
    let num = 2;
    // Por padrão, váriaveis são imutáveis. Para permiti-lás mudarem, mut:
    let mut num = 2;

    // Entre os tipos implementados na std, há a String:
    let mut String::new();
    // Perceba que o :: indica que new é uma função associada do tipo String.
}
