// leia-se: usar lib io da lib std
use std::io;
// parte da std já é adicionada por padrão para todos os programas, chamada prelude.

use rand;

fn main() {

    //
    // rand     para gerar valores aleatórios
    let num = rand::thread_rng() // retorna objeto Rng
        .gen_range(1..101);      // gera número aleatório com a expressão de range como argumento.
                                 // O primeiro número é incluso, porém o segundo não

}
