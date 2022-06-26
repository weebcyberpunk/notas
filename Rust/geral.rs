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
    
    // Rust é orientada a objeto. Para receber input, por exemplo:

    io::stdin()                 // é possível chamar o método na mesma
        .read_line(&mut input)  // linha que o objeto também

    // O que acontece é que stdin() cria uma instância da classe Stdin, que é uma handler do stdin.
    // Depois, chama o método read_line(), que lê uma linha do stdin e salva na string input
    // (passada como reference, os pointers de Rust, mutável)

    //
    // Output é todo feito através de macros declarados em std::fmt:
    //
    format!("Escreve texto formatado para uma string");
    print!("Escreve text formatado no stdout");
    println!("Escreve texto formatado no stdout e pula uma linha");
    eprint!("Escreve texto formatado para o stderr");
    eprintln!("Escreve texto formatado para o stderr e pula uma linha");

    // Para formatar:
    println!("PI é aproximadamente {}.", 3.14);
    // É possível definir os argumentos pelo índice:
    println!("Meu nome é {0}, {1}, {0}.", "Bond", "James");
    // Ou por nomes:
    println!("Meu nome é {sobrenome}, {nome}, {sobrenome}.", 
             sobrenome="Bond", 
             nome="James"
             );
    // mais uma montanha de maneiras... veja a documentação
}
