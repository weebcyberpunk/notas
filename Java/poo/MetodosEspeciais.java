package poo;

public class MetodosEspeciais {
  /* Métodos especiais podem ser:
   *
   * Getter: para pegar um atributo sem interagir diretamente com a classe
   * Setter: para definir um atributo
   * Construct: método usado assim que instanciar o objeto
   */
  
  // Padronização:
  
  setAtributo() {
    this.atributo = "etc";
  }
  getAtributo() {
    this.atributo = "etc";
  }
  
  // O método construtor em Java recebe o mesmo nome da classe, inclusive com a letra maíscula:
  public Classe() {
   this.atributo = "etc"; 
  }
}
