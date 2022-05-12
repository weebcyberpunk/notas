fn main() {

    //
    // Por padrão, váriaveis em Rust são imutáveis. Isso é um dos recursos de segurança da
    // linguagem.
    //
    // Para declarar uma váriavel mutável, basta adicionar mut à declaração:
    let x = 5;     // imutável
    let mut y = 6; // mutável

    // Constantes são basicamente o que se tem com #define em C: valores imutáveis assinalados em
    // compile-time. Para declarar, usa-se const:
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Apesar da característica imutável, Rust possui shadowing de váriaveis. É possível usar o
    // mesmo nome para uma nova váriavel:
    let x = 5;
    let x = 6;

    // A diferença entre utilizar shadowing e váriaveis mutáveis é que o shadowing, criando
    // efetivamente uma nova váriavel, permite mudar o tipo assinalado para o nome:
    let spaces = "   ";
    let spaces = spaces.len();
}
