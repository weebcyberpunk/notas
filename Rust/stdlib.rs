// A stdlib de Rust declara alguns enums interessantes:

// Option
//
// Resolve o problema que a existência de null cria. Rust não tem
// null, para encodar a falta de um valor, se usa Option. Definida
// assim:
enum Option<T> {
    None,
    Some(T),
}
// Ela é tão útil que já é incluída por padrão, sem precisar
// importar nada. As variantes, inclusive, podem ser usadas sem
// especificar o namespace de Option.
fn main() {
    let some_number = Some(5);
    let some_string = Some("a string, definitely");
    let no_number   = Option<isize> = None;
}
// O ponto de se se ter a Option e não null é que, por T e Option<T>
// serem diferentes tipos, você não pode tentar operar entre eles.
// Uma das maiores fontes de bugs com nulls, é que você assume que
// algo não é null mas eventualmente pode ser. Com Option, você é
// forçado a se certificar de que algo não é None antes de tentar
// operar.
