package notas;

public class VariaveiseObjetos {
    public static void main(String[] args) {
        /**No Java, objetos podem ser privados ou públicos
         * Preste atenção, porque o Java diferencia váriaveis de objetos
         * 
         * 
         * Váriaveis:
         * 
         * São declaradas junto com seu tipo:
         */
        int x = 0;
        // Também pode-se fazer com typecast:
        int x = (int) 0;
        // Após a váriavel ser declarada, pode ser mudada a vontade:
        x = 1;
        // Pode-se declarar várias váriaveis juntas:
        int x, y, z;
        x = 0;
        y = 1;
        z = 2;

        // Para declarar objetos:
        String texto = new String("Hello, World!");
        // O nome da classe inicial é wrapper (invólucro)

        /* Famílias, tipos, Classe invólucro, tamanho e exemplo:
         * 
         * ---------------------------------------------------------------
         * | Família | Tipo prim. | Wrapper   | Tamanho      | Exemplo   |
         * |---------|------------|-----------|--------------|-----------|
         * |         |            |           |              |           |
         * | Lógico  | boolean    | Boolean   | 1 bit        | true      |
         * |         |            |           |              |           |
         * | Literal | char       | Character | 1 byte       | 'C'       |
         * | Literal | \\\\\\\\\\ | String    | 1 byte/letra | "Java"    |
         * |         |            |           |              |           |
         * | Inteiro | byte       | Byte      | 1 byte       | 127       |
         * | Inteiro | short      | Short     | 2 bytes      | 32 767    |
         * | Inteiro | int        | Integer   | 4 bytes      | 2 147 483 |
         * | Inteiro | long       | Long      | 8 bytes      | 2^63      |
         * |         |            |           |              |           |
         * | Real    | float      | Float     | 4 bytes      | 3.4e+38   |
         * | Real    | double     | Double    | 8 bytes      | 1.8e+308  |
         * ---------------------------------------------------------------
         * 
         * Declaração dos tipos e classes:
         */

        // Int
        int idade = 3;
        int idade = (int) 3;
        Integer idade = new Integer(3);

        // Float
        float salario = 1845.54f;
        float salario = (float) 1845.54f;
        Float salario = new Float(1845.54f);

        // Char (uma só letra, obrigatoriamente aspas simples):
        char letra = 'c';
        char letra = (char) 'c';
        Character letra = new Character('c');

        // Boolean
        boolean casado = false;
        boolean casado = (boolean) false;
        Boolean casado = new Boolean(false);

        /**Conversão
         * 
         * Tipos não são convertidos facilmente
         * Para converter, usar métodos:
         */
        String valor = Integer.toString(idade);
        Integer numero = Integer.parseInt("3");
        Float numero = Float.parseFloat("3.5");

    }
}
