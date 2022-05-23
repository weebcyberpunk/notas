fn main() {

    /*
     * Slices são como referências, referenciando apenas parte de uma coleção.
     *
     * Ao precisar tratar parte de uma coleção, como uma String, normalmente se salvaria apenas o
     * índice do começo do trecho desejado e o do final, porém, por não estarem atrelados à String
     * em si, caso ela seja modificada, podemos ter erros, além de precisarmos de duas variáveis.
     *
     * Slices de Strings resolvem esses problemas em Rust. Tudo em nome da performanceesegurança:
     */
    let s = String::from("hello world");
    let hello = &s[0..5];   // o slice hello vale "hello"
    let world = &s[6..11];  // o slice world vale "world"
    /*
     * Os slices são referências que tem um valor de tamanho. O slice world por exemplo é uma
     * referência à posição 6 de s, com tamanho de 5 bytes (caso use UTF-8, o programa sai com erro
     * caso o slice corte no meio de um caráctere multibyte).
     *
     * Vale notar que o tipo de um slice é &str
     *
     * String literals (hardcoded) são slices também, apontando para um local específico do
     * binário.
     *
     * É extremamente útil receber parâmetros como slices e não referências à Strings simplesmente
     * de porque caso se tenha uma String, pode passar tanto a referência quanto um slice, e se
     * tiver um slice, pode passá-lo diretamente.
     */

    /*
     * É claro que outros tipos também possuem slices, sendo representados da mesma forma &[..].
     */
}
