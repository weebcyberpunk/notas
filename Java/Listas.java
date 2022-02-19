// Para criarmos arrays com tamanho variável, precisamos usar a classe ArrayList do java.util:
import java.util.ArrayList;

public class Listas {
  public static void main(String[] args) {
    // Para criarmos um arraylist, fazemos assim (eu sei que não parece Java):
    ArrayList<Integer> numbers = new ArrayList<Integer>();
    ArrayList<String> names = new ArrayList<String>();
    // Perceba que Integer é substituído pela classe que você quer seus objetos
    
    // Para adicionarmos e removermos elementos do array:
    numbers.add(1); // [1]
    numbers.add(2); // [1, 2]
    numbers.add(3); // [1, 2, 3]
    
    numbers.remove(1); // [1, 3]
    
    // Perceba que ele irá remover o elemento número 1, e não o elemento cujo valor é 1
    // Para adicionar com o índice, use o índice ANTES do valor:
    
    names.add(0, "GG");
    names.add(0, "Miku");
    
    // Tamanho do array
    int size = names.size();
    // Limpar array
    names.clear();
    // Procurar última ocorrência no array
    names.lastIndexOf("GG");
    // Get para o item na posição
    names.get(0);
  }
}
