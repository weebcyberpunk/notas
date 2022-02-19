public class TratamentoDeExcecoes {
	/*
	 * O tratamento de exceções em Java é feito através de cinco palavras-chave:
	 *
	 * try
	 * catch
	 * throw
	 * throws
	 * finally
	 *
	 * O try é para criar um bloco que será tentado. Caso dê erro, ele irá lançar uma exceção (throw)
	 * Exceções também podem ser lançadas manualmente com throw
	 * Para tratar a exceção lançada, use o bloco catch
	 * Para realizar código caso o try funcione, use finally
	 *
	 * Para um método retornar uma exceção, use throws
	 *
	 * Exemplo:
	 */
	public static int divide(int x, int y) throws ArithmeticException {
		return(x / y);
	}

	public static void main(String[] args) {
		try {
			int x = divide(7, 0);
		} catch(ArithmeticException error) {
			System.out.println("Division by zero");
		}
	}
}
