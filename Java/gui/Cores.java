import java.awt.Color

public class Cores {
  // Cores são feitas com a classe Color do awt
  Color color = new Color(color);
  
  // O construtor aceita constantes como Color.BLUE, Color.BLACK, etc, ou RGB, ou hexadecimal
  
  // Para setar cores, usarei de exemplo JLabel.setForeground():
  JLabel label = new JLabel();
  
  label.setForeground(Color.BLACK); // com constante
  label.setForeground(new Color(0, 0, 0)); // com RGB
}
