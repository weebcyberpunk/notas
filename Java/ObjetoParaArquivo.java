import java.io.*;

public class ObjetoParaArquivo {
	public static void main(String[] args) {
		File file = new File("file.file");
		String string = new String("Hello, World!");
		Object object = null;

		try {
			FileOutputStream stream = new FileOutputStream(file);
			ObjectOutputStream objStream = new ObjectOutputStream(stream);
			objStream.writeObject(string);
			objStream.flush();
		} catch(IOException exception) {
			System.out.println(";(");
		}

		try {
			FileInputStream stream = new FileInputStream(file);
			ObjectInputStream objStream = new ObjectInputStream(stream);
			object = objStream.readObject();
		} catch(IOException exception) {
			System.out.println(";( io exception");
		} catch(ClassNotFoundException exception) {
			System.out.println(";( class not found");
		} finally {
			System.out.println(object);
		}
	}
}
