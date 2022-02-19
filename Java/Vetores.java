package notas;

public class Vetores {
  public static void main(String[] args) {
    /* Vetores são váriaveis compostas
     *
     * Todo vetor é um objeto
     * 
     * Para declarar um vetor em Java:
     */
    int conjunto[] = new int[4]; // Dessa forma, o vetor terá 4 posições (do zero ao 3)
    int[] conjunto = new int[4]; // Também é possível fazer assim, não muda nada
    // Para adicionar valores às posições dos vetores:
    conjunto[0] = 3;
    conjunto[1] = 5;
    conjunto[2] = 1;
    conjunto[3] = 7;
    // Etc
    
    // Para declarar já sabendo os valores do vetor:
    int conjunto[] = {3, 5, 1, 7};
    
    // Sendo objetos, vetores possuem métodos e atributos. O único atributo dos vetores é o length, que é o tamanho do vetor
    
    // Métodos (Precisa importar o java.util.Arrays)
    Arrays.sort(conjunto); // Coloca o conjunto em ordem
    int posicao = Arrays.binarySearch(conjunto, valor); // posicao recebe o valor da posição onde o valor está no conjunto (utiliza busca binária para fazer). Caso o valor não exista no vetor, retorna -valor, pois não existem índices negativos.
    Arrays.fill(conjunto, valor); // Coloca todas as posições de um vetor como sendo o valor 
  }
}
