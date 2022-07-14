// Output de debug pode ser usado com os macros padrões adicionando
// :? às chaves:

#[derive(Debug)] // a nossa struct precisa derivar o trait Debug
struct Rect {
    width: isize,
    height: isize,
}

fn main() {
    let rect = Rect {
        width: 5,
        height: 3
    };
    println!("{:?}", rect);

    // o macro dbg! printa a expressão e o resultado para o stderr,
    // devolvendo depois a posse da expressão
    let num = dbg!(5 + 2);
    dbg!(&rect); // não queremos que dbg! tome posse de rect
}
