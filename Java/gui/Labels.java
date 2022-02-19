import javax.swing.*;

public class Labels {
  public static void main(String[] args) {
    JFrame window = new JFrame();
    ImageIcon icon = new ImageIcon("/usr/share/icons/labels.jpeg");
    
    // Um label é um componente para mostrar texto ou imagem na tela (ou os dois)
    JLabel label = new JLabel("Hello, World!"); // o texto é opcional, pode ser colocado depois com o setter
    
    // Métodos úteis:
    label.setText("Hello!"); // para setar o texto
    label.setIcon(icon); // para setar a imagem, recebe objeto ImageIcon
    label.setIconTextGap(tamanho); // para setar a distância (gap) entre a imagem e o texto
    label.setOpaque(true); // para deixar o label em si opaco
    
    // Cores e fontes:
    label.setForeground(color); // para setar a cor das letras. Ver mais em Cores.java
    label.setBackground(color); // para setar a cor do label em si. O label precisa ser opaco
    label.setFont(font); // para setar a fonte. Ver mais em Fontes.java
    
    // Posicionamento:
    label.setHorizontalTextPosition(JLabel.CONSTANT); // para setar a posição do texto em relação ao próprio label. Aceita CENTER, RIGHT, LEFT
    label.setVerticalTextPosition(JLabel.CONSTANT); // mesma ideia do horizontal. Aceita TOP, CENTER, BOTTOM
    label.setHorizontalAlignment(JLabel.CONSTANT); // mesma ideia dos outros, seta o alinhamento horizontal do texto e imagem para com o label
    label.setVerticalAlignment(JLabel.CONSTANT); // intuitivo
    
  }
}
