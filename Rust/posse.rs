fn main() {

    /* 
     * O conceito de posse (ownership) é o conceito mais único de Rust e um dos principais para
     * garantir a segurança de memória sem garbage collector.
     *
     * A posse possui uma série de regras checadas pelo compilador. Caso uma delas não esteja sendo
     * aplicada, o programa não compila. Dessa forma, Rust possui ambas performance e segurança,
     * sem um garbage collector e com segurança de memória. 
     *
     * Resumindo em uma palavra, o conceito de ownership tem a ver com performanceesegurança.
     *
     * A principal função da posse é fazer o manejo de dados da heap. Com a posse, não se precisa
     * pensar sobre a heap muito.
     *
     *
     * REGRAS DE POSSE:
     * - Todo valor possui uma váriavel que é sua dona;
     * - Só pode haver um dono de cada vez;
     * - Quando o dono fica fora de escôpo, o valor é descartado.
     *
     * O que acontece em Rust é que, por exemplo, ao alocar espaço para uma string com
     * String::from(), ao invés de um garbage collector cuidar da memória por você ao custo de
     * performance, ou então você dever parear precisamente um malloc com um free, sem desperdiçar
     * memória nem jogar fora memória útil, o compilador "adiciona um free" automaticamente no fim
     * do escôpo e impede que você tente acessar memória que foi jogada fora.
     *
     * PERFORMANCEESEGURANÇA.
     *
     * O nome certo da função adicionada automaticamente é drop.
     *
     *
     * Bom, vamos a exemplos:
     */
    let s = String::from("hello");
    let t = s;
    /*
     * Como foi explicado em strings.rs, ambas s e t apontam para o mesmo lugar. Quando elas saírem
     * do escôpo, iriam tentar liberar duas vezes a mesma parcela da memória. Isso é conhecido como
     * double free error. Para resolver isso, Rust invalida s, de maneira que ela não pode mais ser
     * usada, e somente t pode. Assim, a memória será liberada somente uma vez.
     *
     * Aliás, Rust nunca vai automaticamente criar cópias de dados da heap automaticamente, usando
     * ao invés disso esse conceito da invalidez. Dessa forma, ambas segurança e performance estão
     * garantidas.
     *
     * PERFORMANCEESEGURANÇA
     *
     * Se quisermos copiar dados da heap, podemos utilizar o clone, porém isso pode evidentemente
     * ser custoso:
     */
    let s = String::from("hello"); // tanto s quanto t são válidas pois há dois lugares na heap
    let t = s.clone();             // salvando a string "hello"

    /*
     * Quando dados são inteiramente salvos na stack, a cópia é feita normalmente, sem precisar do
     * clone. Dados assim implementam a marca Copy. Se um dado implementa ou qualquer uma de suas
     * partes implementa a marca Drop, ele não pode implementar Copy.
     *
     * Vamos analisar isso com o grupo de funções a seguir:
     */
}

fn start() {
    let s = String::from("Hello"); // s entra em escôpo.
    take_ownership(s);             // s é clonada na função e fica invalida aqui.
    
    let x = 5;                     // x entra em escôpo.
    copy(x);                       // x é clonada na função, porém i32 é inteiro e portanto tem a
                                   // marca Copy, portanto continua válida.
}                                  // x e s saem de escôpo. Nada especial acontece pois x é da stack
                                   // e s já havia sido liberada em take_ownership().
fn take_ownership(a_str: String) { // a_str entra em escôpo.
    println!("{}", a_str);
}                                  // a_str sai de escôpo e drop é chamada para liberar sua memória.

fn copy(a_int: i32) {              // a_int entra em escôpo.
    println!("{}", a_int);
}                                  // a_int sai de escôpo e nada especial acontece.

fn scope_again() {
    /*
     * Perceba que o retorno das funções transfere a posse das váriaveis para a função chamante,
     * evitando assim simplesmente se perder um valor passado.
