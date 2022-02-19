package notas;

public class Repeticao {
    public static void main(String[] args) {
        // Repetição enquanto simples
        int vez = 0;
        while (vez < 5) {
            // bloco
        }

        // break interrompe e continue pula o resto do bloco e continua
        while (vez == 0) {
            if (vez == 0) {
                continue;
            } else {
                break;
            }

        // Repetição com teste no final:
        do {
            // bloco
        } while (vez < 5);

        // Repetição para
        for (int x = 0; x < 5; x++) {
            // bloco
        }
        /**Na repetição para, tem-se que:
         * 
         * Para x = 0;
         * Enquanto x < 5;
         * Incremente x
         * 
         * Assim, x é 0, enquanto ele for menor que 5, irá se somar 1 a ele e fazer o bloco
         * 
         * É uma boa ideia notar que isso é uma estrutura enquanto simplificada, e não para
         *
         *
         * É possível usar o for com vetores também:
         */
         int num[] = {3, 5, 7};
         for(int valor: num) {
             // bloco
         }
        }
    }
}
