#include<stdio.h>

int main() {
	// Funções são declaradas tal qual a função main é declarada, com o tipo de retorno e etc
	// Porém, funções podem omitir várias coisas. A menor das funções é a seguinte:
	dummy() {}
	// Porém ela não faz nada nem retorna nada. Aliás, quando o tipo de retorno não é definido, presume-se int
	
	// Quanto ao retorno, caso a função não retorne nada explicitamente, ela irá retornar ao chamador, mas sem valor. Da mesma forma, um return pode retornar nada
	
	// Quando for usar uma função de outro arquivo, é recomendável que escreva ela abaixo dos #include. Caso contrário, a compilação provavelmente funcionará porém o compilador lhe alertará sobre a função ser implicitamente declarada. O problema de funções implicitamente declaradas é que elas são consideradas como sua primeira aparição no arquivo
	// Para compilar C com mais de um arquivo, com as funções declaradas, pode-se entrar os arquivos na ordem que quiser
	
	// Ao compilar mais de um arquivo, o compilador irá gerar um único arquivo alvo "a.out" e mais um arquivo para cada função, com a extensão .o
	// Caso precise recompilar uma função, pode simplesmente recompilar o .c dessa função com o .o das outras

	// Falando da declaração agora: se as funções não retornam nada, declare com retorno void. Por boa prática só, porque funciona sem

	// Perceba que o retorno irá parsear automaticamente o que ele retorna. 
	// Exemplo: se a função retorna int, porém no final o return() possui um double, esse double será parseado para int automaticamente

	return(0);
}
