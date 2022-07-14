fn main() {

    // O tipo String representa (pasmem) strings em Rust.

    /*
     * Uma String que não seja um literal (string hard-coded) é, obviamente, salva na heap. Na
     * stack, há um ponteiro para a string na heap, um valor de capacidade e um de tamanho.
     *
     * Por isso, Strings não devem ser tratadas como tipos comuns. Ex:
     */
    let mut x = 5; // aqui, o valor 5 é copiado para a outra váriavel
    let mut y = x; // de forma que temos uma cópia dele

    let mut s = String::from("hello"); // aqui, somente os valores da stack são copiados, de maneira
    let mut t = s;                     // que temos só um texto na heap

    /*
     * Métodos úteis:
     */
    String::new();                 // cria uma string na heap, manejada pela linguagem
    String::from("literal");       // cria uma string na heap iniciada pelo argumento literal passado
                                   //
    string.push_str("new text");   // append para a string (deve ser mutável)
    string.trim(); // remove espaços em branco e \n ao redor da string.
    string.parse(); // transforma strings em diferentes tipos de números.
}
