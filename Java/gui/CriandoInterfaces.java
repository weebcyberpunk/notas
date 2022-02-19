import javax.swing.*;

/* 
 * PRIMEIRO DE TUDO, ENTENDA: JAVA SWING E AWT SÃO ORIENTADAS A EVENTOS
 *
 * AWT é a tecnologia mais primitiva de criação de interfaces em Java
 * Swing é baseada na AWT e herda muitas de suas classes
 */

// A melhor forma de criar uma interface é extender a classe JFrame e depois chamar a filha no seu método main
public class CriandoInterfaces extends JFrame {
  
  /* 
   * Para criar JFrames com uma classe filha, coloca-se a criação no construtor. 
   * Para criar direto no método main, instancia-se a classe e depois usa os métodos
   */
  public CriandoInterfaces() {
    // Aqui mostrarei algumas coisas úteis:
    this.setDefaultCloseOperation(JFrame.JFRAME_CONSTANT); // usa constantes para definir o jeito que a janela responderá ao clicar no X. EXIT_ON_CLOSE é para fechar o programa ao clicar.
    this.setSize(420, 420); // tamanho da janela
    this.setTitle("Hello, World!"); // título da janela
    this.setVisible(true); // por padrão, janelas são invisíveis. Sete-as como visíveis
    
    // Para adicionar componentes à sua janela, use:
    JLabel label = new JLabel("Hello, World!");
    this.add(label);
    
    // Métodos úteis de JFrames:
    this.setLayout(LayoutManager layout); // seta o layout manager da janela. Coloque em null se quiser acertar as posições dos componentes na mão
    
    // Métodos úteis de componentes:
    label.setBounds(x, y, width, height); // seta a posição do componente se não tiver um layout manager
  }
}
