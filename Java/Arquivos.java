import java.io.File;
import java.io.IOException;
import java.util.Scanner;

public class Arquivos {
	public static void main(String[] args) {
		/*
		 * Para manipular arquivos, deve-se criar um objeto File de java.io
		 *
		 * É extremamente recomendável que se manipule os arquivos em blocos de try
		 */
		try {
			File file = new File("nome-do-arquivo");

			file.createNewFile(); // cria um arquivo no sistema, retorna erro se não possível

			/*
			 * Para escrever em arquivos, deve-se criar um objeto FileWriter:
			 */
			FileWriter writer = new FileWriter("nome-do-arquivo");
			// para escrever:
			writer.write("Hello, World! \n");
			// para fechar o arquivo (EXTREMAMENTE IMPORTANTE!):
			writer.close();

			/*
			 * Para apagar arquivos:
			 */
			file.delete();

			/*
			 * Para ler de arquivos, recomenda-se usar o Scanner:
			 */
			 Scanner scanner = new Scanner(file);
			 scanner.nextLine() // retorna próxima linha, perceba que usar isso várias vezes retorna linhas diferentes
			 // LEMBRE-SE DE FECHAR O SCANNER!

		} catch(IOException exception) {
			System.out.println("Um erro ocorreu :(");
		}
	}
}
