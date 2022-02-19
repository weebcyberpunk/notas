package poo;

    /* 
     * O encapsulamento é o primeiro pilar da Orientação à Objeto 
     *
     * O código encapsulado protege a si mesmo de um programador que queira modificá-lo. Além disso, ele fornece um "padrão":
     * não importa onde você for utilizá-lo, ele sempre se comportará igual, e não importa como ele se comporta internamente, 
     * não irá afetar seu programa desde que ele continue fazendo seu trabalho
     *
     * O conceito de encapsular é definido assim: 
     * "Ocultar partes independentes da implementação permitindo construir partes invisíveis ao mundo exterior."
     * 
     * Você não precisa saber pra que serve aquela parte, só precisa saber o que ela te fornecerá
     * 
     * O nome da parte que vocẽ interage com um objeto encapsulado é interface
     *
     * Perceba que o encapsulamento não é obrigatório, mas é uma boa prática
     *
     *
     * As vantagens de se encapsular são:
     *
     * 1- Tornar mudanças invisíveis. Uma mudança na parte encapsulada não irá modificar o resto do software
     * 2- Facilitar a reutilização de código. Modularização. Acho que isso é auto-explicativo
     * 3- Reduzir efeitos colaterais. Evita que um programador altere um objeto e mude seu funcionamento prejudicialmente
     * 
     * A interface deve ter somente métodos, e não atributos
     * Inclusive, na interface, os métodos são abstratos, eles só estão previstos ali, porém não implementados
     * 
     * Na interface, os métodos são geralmente definidos como públicos. O que faz bastante sentido
     * 
     * Para encapsular um objeto, deixe todos os atributos privados. No máximo protegidos
     *
     * 
     * Geralmente, se define a interface de um objeto em um arquivo a parte, e os métodos são definidos como abstratos
     *
     * Quando uma classe possui uma interface, diz-se que ela implementa essa interface, e você será obrigado a sobreescrever
     * os métodos da interface nessa classe
     *
     *
     * Para criar uma interface Java:
     */

public interface Controlador {
    public abstract void ligar();
    public abstract void desligar();
    public abstract void aumentarVolume();
    public abstract void diminuirVolume();
    public abstract void play();
    public abstract void pause();  
}

// Para implementar a interface a uma classe:

public class Controle implements Controlador {
  // Override serve para sobreescreever a programação que não havia sido sequer feita
  @Override
  public void ligar() {
    
  }
  
  @Override
  public void desligar() {
  
  }
  
  // Continua por todos os métodos
}
