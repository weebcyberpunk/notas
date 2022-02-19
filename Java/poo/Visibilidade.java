package poo;

public class Visibilidade {
  /* A visibilidade define quem pode usar certo método ou atributo.
   *
   * A visibilidade padrão do Java é chamada de "pública para pacote" (isso só trará problemas se trabalhar com mais de um pacote sem definir a visibilidade)
   * Por boa prática, sempre defina a visibilidade
   *
   * Além disso, ela pode ser:
   */
  
  public
    // Todas as classes podem usar

  private
    // Somente a classe atual pode usar
    
  protected
    // Somente a classe atual e suas subclasses podem usar. O método main consegue usar se estiver utilizando a classe
}
