package notas;

// Importações são feitas assim
import java.util.*; // Utilidades do Java
import java.math.*; // Funções matemáticas
import java.applet.*; // Para criação de applets
import java.net.*; // Redes
import java.awt.*; // Interfaces gráficas com aparência que varia de acordo com o OS

// Bibliotecas extendidas

import javax.swing.*; // Interfaces gráficas em janela
import javax.media.*; // Tratamento de mídia
import javax.fxml.*; // Interfaces gráficas não necessariamente em janela

public class Bibliotecas {
    public static void main(String[] args) {
        /* O pacote essencial do Java é o java.lang
         * 
         * Alguns pacotes úteis estão listados nos imports, aqui a explicação de alguns deles:
         */

        /* java.util
         * 
         * Pacote de utilidades, possui alguns recursos interessantes:
         */
         
        // classe Scanner, para entrada de dados, exemplo:

        Scanner keyboard = new Scanner(System.in);
        String text = keyboard.nextLine(); // Pegar próxima linha
        Float numero = keyboard.nextFloat(); // Pegar próximo float
        Integer numero = keyboard.nextInt(); // Pegar próximo inteiro
        keyboard.close();

        // Classe Date, a data e hora do sistema:

        Date today = new Date();
        
        // Classe Array, para manipular arrays (ver mais em vetores)
        
        // Classe ArrayList, para criar arrays de tamanho variável (ver mais em Listas)
        
        // Classe Random, para mecher com números aleatórios
        
        Random random = new Random();
        int num = random.nextInt(x); // x é o número limite
        double num = random.nextDouble();
        // etc

        /* java.math
         * 
         * Pacote de funções matemáticas:
         */

        // Retornar valor de Pi:
        Double pi = Math.PI;
        // Potência:
        double potência = Math.pow(2, 2);
        // Raíz quadrada
        double raiz = Math.sqrt(4);
        // Raíz cúbica
        double raiz = Math.cbrt(8);
        // Valor absoluto (o exemplo retornará 10)
        int absoluto = Math.abs(-10);
        // Arredondamento truncado
        double redondo = Math.floor(3.3618);
        // Arredondamento acima
        double redondo = Math.ceil(3.3618);
        // Arredondamento padrão
        double redondo = Math.round(3.3618);
        // Gerador de número aleatório
        double aleatório = Math.random();

        // Utilizando o random na prática, para gerar número inteiro:
        double aleatório = x + Math.random() * (y - x);
        
        /* java.time
         *
         * Pacote de funções de tempo
         */
        
        // Retornar o presente instante no timestamp em segundos
        Long timestamp = Instant.now().getEpochSecond();
    }
}
