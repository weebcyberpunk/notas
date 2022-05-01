fn cargo() {

    // O Cargo é o build system de Rust. Utilizar só o rustc é possível, mas nós somos mais
    // tecnológicos que isso.
    //
    // Para criar um novo projeto com o Cargo, usa-se '$ cargo new <nome>'.
    //
    // Cargo usa um arquivo escrito em TOML (Tom's Obvious, Minimal Language) para configurar
    // projetos. É literalmente a coisa mais simples do mundo de ler e entender.
    //
    // O código em si do projeto deve estar em src. Organize-se.
    //
    // Após a primeira build, o cargo gerará um arquivo Cargo.lock que mantém informações sobre as
    // exatas dependências usadas. O arquivo é automaticamente gerenciado pelo cargo.
    //
    // Para buildar, '$ cargo build'. Para buildar e automaticamente rodar o binário resultante, '$
    // cargo run'.
    //
    // O Cargo também possui um comando útil para checar se o programa compila porém sem compilar,
    // '$ cargo check'.
    //
    // Para compilar o binário que o usuário final vai usar, o Cargo possui a bandeira '--release'.
    // Ela ativa certas otimizações que demoram mais para compilar, porém tornam o resultado final
    // mais (pasmem) otimizado.
    //
    // Build comuns são colocadas em target/debug e releases em target/release.

}
