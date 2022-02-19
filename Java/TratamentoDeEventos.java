import java.util.EventObject;
import java.util.TooManyListenersException;

public class TratamentoDeEventos {
	/*
	 * Eventos são gerados por sources e tratados com listeners
	 * Sempre que um evento for gerado, sua source enviará uma cópia do evento para os listeners
	 *
	 * A forma geral de se criar um listener para uma source é:
	 */
	public void addTypeListener(TypeListener listener) {
		// perceba que type é trocado pelo tipo de evento
		// para um evento de teclado pressionado, por exemplo, é addKeyLister
	}
	public void addTypeListener(TypeListener listener) throws TooManyListenersException {
		// caso haja limite de listeners para a source, dispara a exceção
	}

	// Para remover um listener, chame removeTypeListener();
	
	/*
	 * IMPORTANTE: perceba que os métodos de adição e remoção de listeners são providos pela classe da source em si
	 *
	 *
	 * A superclasse de todo evento é java.util.EventObject. Seu construtor:
	 */
	EventObject event = new EventObject(Object src); // perceba que src é a source

	// EventObject define um getSource() e um toString()
}	
