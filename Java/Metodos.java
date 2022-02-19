public class Metodos {

    /* Falando um pouco sobre o método main, ele é sempre o método executado por padrão
     * 
     * (o public static void main(String[] args) que tá lá embaixo)
     * 
     * main é um método que não retorna valor, recebe o parâmetro args que é um vetor de String, é estático e público
     */

    /* Sobre os métodos são... métodos
     * 
     * Ao declarar um método:
     * 
     * Sobre o retorno:
     * void indica que o método não retornará nenhum valor (portanto ele é um procedimento)
     * Para retornar, é só tirar o void
     * 
     * Sobre a dinâmica:
     * static indica que o método é estático, ele é método apenas funcional na classe, e não é instânciado dentro de objeto. Entre outras coisas, não se pode chamar um método não estático dentro de um método estático
     * 
     * Sobre o acesso:
     * public indica que o método é público, portanto pode ser acessado de qualquer código em qualquer lugar
     * private indica método privado
     * protected indica método protegido
     */
    static void soma(int x, int y) {
        int r = x + y;
        System.out.println(r);
    }

    /**Métodos que retornam (funções) devem ter seu tipo de retorno especificado
     * 
     */
    static int somaRetorno(int x, int y) {
        int r = x + y;
        return r;
    }
    
    public static void main(String[] args) {
        soma(40, 2);

        int r = somaRetorno(40, 2);
    }
}

/**Criando múltiplas classes
 * 
 * É possível criar métodos fora do seu programa principal. O seu arquivo de métodos então terá somente a classe principal e os métodos, sem o método main:
 * 
 */

public class Operacoes {
    public static String contador(int i, int f) {
        String s = " ";
        for(int c = 0; c <= f; c++) {
            s = c + " ";
        }
        return s;
    }
}
// No seu arquivo principal:

public class Programa {
    public static void main(String[] args) {
        System.out.print(Operacoes.contador(40, 2));
    }
}
