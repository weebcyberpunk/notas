#include<stdio.h>

int main() {
	// Váriaveis que estejam dentro de uma função só podem ser acessadas por essa função
	// Porém, váriaveis externas a funções podem ser acessadas do momento em que foram declaradas até o final do arquivo

	// Ainda é possível acessar váriaveis declaradas em um arquivo, de outro. No arquivo onde ela foi definida, define-se normalmente:
	int value;
	// E no arquivo em que ela será acessada, declara-se assim:
	extern int value;
	// Perceba que o tamanho de arrays deve ser definido na definição da váriavel, mas não precisa constar em suas declarações
	
	// Outra coisa possível de ser feita é criar um arquivo de cabeçalho que centralizará as declarações. Para isso, deve-se criar um arquivo com a extensão .h (de "header") e incluí-lo com #include nos outros arquivos:
	#include "declarações.h"
	// Perceba que, dessa forma, não é preciso declarar as váriaveis e funções nos outros arquivos: é preciso iniciá-las em seus respectivos arquivos, porém sem declarar em outros 

	return(0);
}
