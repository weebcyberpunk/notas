public class Polimorfismo {
  /* A ideia do polimorfismo é que o mesmo nome represente vários comportamentos diferentes
   *
   * Isso é, o mesmo getAtributo() de duas classes que vieram da mesma mãe, podem apresentar comportamentos diferentes
   *
   * Com o polimorfismo, é preciso prestar atenção na assinatura dos métodos. Isso é, a quantidade e o tipo dos parâmetros de cada método
   *
   * Por exemplo, os seguintes métodos tem a mesma assinatura:
   */
  public static int dividir(int num1, int num2) {
    return(num1 / num2);
  }
    public static float dividir(int num1, int num2) {
    return(num1 / num2);
  }
  /* Mesmo que um retorne int e o outro float, não é possível diferenciar um ou outro no código, pois os dois se chamam dividir() e recebem dois int
   *
   * Existem diferentes tipos de polimorfismo. O mais comum é o de sobreposição
   *
   * A sobreposição ocorre quando temos métodos com a mesma assinatura em duas classes de uma mesma mãe. Para que os métodos das classes sobreponham o
   * método da mãe, deve-se marcar o método com @Override, como já foi visto algumas vezes
   *
   * Outro tipo de polimorfismo é o de sobrecarga, que ocorre quando se tem diversos métodos com o mesmo nome na mesma classe, porém parâmetros diferentes
   *
   * Perceba que o que vale é a assinatura do método: esses dois métodos são idênticos em assinatura:
   */
  public static somar(int num1, int num2) {
    return(num1 + num2);
  }
  public static somar(int x, int y) {
    return(x + y);
  }
  
  // Em contrapartida, esse não é:
  
  public static somar(float num1, float num2) {
    return(num1 + num2);
  }
  
  // Os dois métodos seguintes também NÃO SÃO idênticos, pois um informa primeiro o float e depois o int, e o outro é ao contrário:
  
  public static somar(float num1, int num2) {
    return(num1 + num2);
  }
  public static somar(int num1, float num2) {
    return(num1 + num2);
  }
}
