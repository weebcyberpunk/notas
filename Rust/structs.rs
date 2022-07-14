// Para criar structs, é semelhante à C:

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // fields não precisam estar em ordem
    let mut user1 = User {
        email: String::from("weebcyberpunk@gmail.com"),
        username: String::from("weebcyberpunk"),
        active: true,
        sign_in_count: 1,
    };

    // o acesso a fields é feito com pontos
    user1.username = String::from("gg");
    // note que a struct inteira deve ser mutável

    // para basear uma nova instância de struct em outra:
    let user2 = User {
        email: String::from("flavinho@dopneu.com"),
        username: String::from("shaolin_matador_de_porco"),
        ..user1 // note que deve estar no final
    };
    // Perceba que se user2 pegasse os valores String que
    // pertencem a user1, user1 sairia de escopo pois são
    // valores movidos e não copiados.
}

// Structs podem ser retornadas de expressões, então 
// um construtor pode ser feito assim:
fn new_user(email: String, username: String) -> User {
    User {
        email,    // note o açúcar: se o parâmetro tem o
        username, // nome do field, não precisa especificar
        active: true,
        sign_in_count: 1,
    }
}

// Rust também suporta tuple structs, que são structs com
// valores sem nome
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn create_dot() {
    let black  = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // Perceba que apesar de black e origin serem "literalmente
    // a mesma porra" eles são de tipos diferentes pois tem duas
    // definições de struct. O compilador de Rust reclamaria se
    // você tentasse reassignar eles entre si.
}

// É possível até ter structs sem data ("unit-like struct"):
struct AlwaysEqual;
fn alwaysequal() {
    let subject = AlwaysEqual;
}

// Também é possível que structs armazenem referências a coisas
// que não possui, porém isso necessita de especificadores de
// lifetime.
