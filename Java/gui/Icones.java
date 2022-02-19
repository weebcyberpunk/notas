import javax.swing.*;

public class Icones {
  public static void main(String[] args) {
    JFrame window = new JFrame();
    
    // Ícones são criados com a classe ImageIcon
    ImageIcon icon = new ImageIcon("rota/para/o/icone");
    
    // Para adicionar os ícones ao seu JFrame:
    window.setIconImage(icon.getImage());
  }
}
