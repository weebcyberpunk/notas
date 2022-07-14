// As enums de Rust são outra herança da programação funcional,
// sendo parecidas com os "algebraic data types" de linguagens
// funcionais como F#, OCaml e Haskell.
//
// Enums definem tipos de maneira diferente de structs. Enums criam
// enumerações: quando se há diferentes, finitas e sabidas variações
// de um tipo. Exemplo, endereços de IP que podem ser tanto V4
// quanto V6:

enum IpAddrKind {
    V4,
    V6,
}

// São tipos, então podemos criar instâncias:
fn main() {
    let four = IpAddrKind::V4; // note que as variações estão no
    let six  = IpAddrKind::V6; // namespace do enum
}

// Pense agora que IPs tem um valor também, o IP em si. Ao invés de
// associar o enum a uma struct IP, podemos fazer com que o próprio
// enum carregue informações:
enum IpAddrKind {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));
}

// MELHOR AINDA. IPv4 é um conjunto de 4 números de 8 bits, mas o v6
// ainda pode ser representado como string:
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));
}

// Enums também aceitam métodos e funções em blocos de impl.
