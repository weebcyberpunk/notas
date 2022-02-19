package notas;

public class Operadores {
    public static void main(String[] args) {
        /* Operadores aritméticos
         * 
         * + soma e concatenação
         * - subtração
         * * multiplicação
         * / divisão
         * % resto da divisão
         * 
         * 
         * Operadores unários
         * 
         * ++ incremento
         * -- decremento
         * 
         * Porém, perceba que:
         */
        int num = 5;
        int valor = 5 + num++; // valor será igual a 10, pois num sofrerá incremento após todas as operações

        num = 5;
        valor = 5 + ++num; // valor será igual a 11

        /* Operador ternário
         * 
         * Avalia se a expressão é verdadeira e atribui um valor com base nisso:
         */
        int x = 1;
        int y = 2;
        int maior = x > y ? x : y; // Caso x > y, maior é x, caso contrário, maior é y

        /**Operadores relacionais
         * 
         * > maior que
         * < menor que
         * >= maior ou igual a
         * <= menor ou igual a
         * == equivalente a
         * != diferente de
         * 
         * O método para verificar a equivalência de objetos é o .equals()
         * 
         * 
         * Operadores lógicos:
         * 
         * && operador E, caso os dois valores true, retorna true
         * || operador OU, caso um dos valores true, retorna true
         * ^ operador OU EXCLUSIVO, retorna true se um for true, porém false se os dois forem true
         * ! operador NÃO, para um único valor. Se não true, será false
         */
    }
}
