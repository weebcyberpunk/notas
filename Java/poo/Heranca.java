public class Heranca {
  /* A herança é o segundo pilar da orientação a objeto
   * 
   * Conceitualmente, a herança permite basear uma nova classe na definição de outra classe já existente
   *
   * A herança é aplicada tanto para as características quanto para os comportamentos. Toda classe mãe concede todas as suas características e comportamentos
   * para suas classes filhas
   *
   * Perceba que uma subclasse pode ser mãe de outra classe, fazendo uma verdadeira árvore genealógica de classes (ou só árvore)
   *
   * Para que uma classe herde características de outra:
   */
  
  public class Student extends Person {
    // Para sobreescrever um método existente na outra classe, usar o override:
    @Override
    public void setAge() {
      
    }
    
    // Para que o construtor da classe filha implemente aspectos da classe mãe:
    public class Student extends Person {
      public Student(name, age) {
        super(name, age);
      }
    }
  }
}
