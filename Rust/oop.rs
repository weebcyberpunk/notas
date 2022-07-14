// A orientação a objeto de Rust é zoada e isso é bom porque eu não
// gosto de orientação a objeto.
//
// Na verdade, os nossos objetos são structs em si.

// Métodos são funções declaradas no contexto de structs, e recebem
// sempre o primeiro parâmetro self.

#[derive(Debug)]
struct Rectangle {
    width: isize,
    height: isize,
}

// impl declara um bloco de implementação, que nos permite declarar
// coisas no contexto da nossa struct.
impl Rectangle {
    fn area(&self) -> isize {
        self.width * self.height
    }

    // No geral, métodos devem receber &self ou &mut self, porque se
    // receber somente self, ele irá tomar a posse do próprio self.
    // Então é boa prática receber &self quando só precisar ler fields
    // da struct e &mut self quando precisar escrever na struct.
    
    // Note que é possível ter métodos com nomes de fields, e isso é
    // bem útil para implementar getters (Rust não os implementa
    // automaticamente):
    fn width(&self) -> isize {
        self.width
    }

    // Métodos que não recebem self são chamados de funções 
    // associadas:
    fn square(size: isize) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 5,
        height: 3,
    };

    // Métodos são chamados assim como se acessa fields das structs:
    let area = rect.area();

    // Funções associadas estão no namespace de seus tipos, então se
    // chama assim:
    let square = Rectangle::square(4);
}

// Note ainda que não existe o operador -> para derreferenciar 
// structs porque Rust implementa DERREFERENCIAMENTO AUTOMÁTICO que
// por acaso também é uma coisa que muita gente gostaria que
// existisse em C.
