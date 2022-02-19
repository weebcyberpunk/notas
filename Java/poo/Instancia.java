package poo;

public class Instancia {
  public static void main(String[] args) {
    // Instanciando objeto de classe definida em DefinindoClasses
    Caneta c1 = new Caneta();
    c1.cor = "Azul";
    c1.ponta = 0.5f;
    c1.tampada = false;
    c1.modelo = "Bic";
    c1.carga = 90;
    
    // Caso peça um atributo que não foi definido para o objeto, retorna null para esse atributo (lembrando que o método status foi definido na classe Caneta)
    c1.status();
  }
}
