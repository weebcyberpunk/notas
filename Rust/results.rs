fn main() {
    
    // Results são objetos utilizados para fazer o handle de erros.
    //
    // Há várias variações, porém no geral, results podem ser 'Ok' ou 'Err'.
    //
    // O método expect é usado para tratar Results de maneira simples:
    sdt::io::Stdin()
        .read_line(&mut input)
        .expect("Input falhou.");
    // O que ocorre é que, caso o Result retornado por read_line() for Ok, expect() irá retornar o
    // valor que ele guarda. Caso o Result seja Err, expect irá encerrar o programa com a mensagem
    // de erro passada de argumento.

    // Um dos motivos de Rust ser segura é que, caso haja um Result sendo criado em algum lugar sem
    // um handler como expect, o compilador irá te avisar que você possívelmente tá deixando uma bosta acontecer
    // sem cuidar dela.

}
