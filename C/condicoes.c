#include<stdio.h>

int main() {
	// Em C, if e else são construídos da seguinte maneira:
	int x = 1;
	if(x > 0) {
		printf("maior");
	} else {
		printf("menor");
	}

	// Ainda é possível fazer if com um só comando sem bloco:
	if(x > 0) prinf("maior");

	// else if:
	int x = 1;
	if(x == 0) {
		prinft("zero");
	} else if(x == 1) {
		printf("um");
	} else {
		printf("maior que um");
	}


	// É possível usar switch para comparar constantes inteiras:
	
	switch(1 - 1) {
		case 0:
			prinft("zero");
			break;
		case 1:
			prinf("um");
			break;
		default:
			printf("maior que um");
			break;
	}
	// Perceba que, caso um case anterior faça um seguinte ser verdadeiro, o seguinte também será executado a não ser que se use break. Por boa prática, sempre coloque break, mesmo na default, exceto que seja necessário que não.
	// Perceba também que é possível colocar vários cases seguidos para a mesma condição:
	switch(1 - 1) {
		case 0: case 2:
			printf("zero ou dois");
		case 1: case 3:
			printf("ímpar");
	}

	return(0);
}
