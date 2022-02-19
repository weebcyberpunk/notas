#include<stdio.h>

int main() {
	// Para mexer com arquivos em C, usa-se a própria biblioteca stdio
	// Primeiro abre-se o arquivo:
	FILE *fp;
	fp = fopen("nome", "modo");
	// Perceba que fp é um pointer para o arquivo
	// No nome do arquivo, evidentemente encontra-se o nome do arquivo
	// No modo, coloca-se o modo de abertura, que pode ser "r" (leitura), "w" (escrita) e "a" (adição)
	// Alguns sistemas diferenciam arquivos de texto e binários. Para abrir em modo binário, coloca-se "b" junto do modo
	// Para fechar o arquivo:
	fclose(fp);
	// Abrir em modo de escrita ou adição um arquivo inexistente irá criá-lo, porém abrir em modo de leitura irá dar erro
	// fopen retorna null em caso de erro
	//
	// As seguintes funções funcionam exatamente igual a scanf e printf, porém o primeiro argumento é o pointer para o arquivo:
	fscanf(*fp, string);
	fprintf(*fp, string);

	return(0);
}
