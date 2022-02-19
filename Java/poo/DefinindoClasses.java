package poo;

// A Classe pode ser instanciada de dentro do pacote. Para ver como instanciar, ver Instância
public class Caneta {
  String modelo;
  String cor;
  float ponta;
  int carga;
  boolean tampa;
  
  void status() {
    // this é o equivalente do Java ao self do Python. É a auto-referência
    System.out.println("Modelo: " + this.modelo);
    System.out.println("Carga: " + this.carga);
    System.out.println("Cor: " + this.cor);
    System.out.println("Ponta " + this.ponta);
    System.out.println("Está tampada? " + this.tampa);
  }
  
  void rabiscar() {
    if (this.tampada == true) {
      System.out.println("ERRO");
    } else {
      System.out.println("Rabiscando");
    }
  }
  
  void tampar() {
    this.tampada = true;
  }
  
  void destampar() {
    this.tampada = false;
  }
}
