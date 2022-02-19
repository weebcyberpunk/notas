package notas;

public class Condicoes {
    public static void main(String[] args) {
        int x = 0;
        int y = 0;
        // Condição simples
        if (x > y) {
            // bloco
        }

        // Condição composta
        if (x > y) {
            // bloco
        } else if (x < y) {
            // bloco
        } else {
            // bloco
        }

        // Condição de múltipla escolha. Verifica o valor de um int:
        int sobremesa = 0;
        String gosto = new String();
        switch (sobremesa) {
            case 0:
                gosto = "muito bom";
                break;
            case 1:
                gosto = "bom";
                break;
            default:
                gosto = "desconhecido";
        }
    }
}
