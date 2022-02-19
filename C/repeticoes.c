#include<stdio.h>

int main() {
	// while:
	while(1 == 1) {
		printf("1 equivale a 1");
	}

	// for:
	int x;
	for(x = 0; x < 5; x++) {
		printf("x menor que 5");
	}
	// Perceba que qualquer expressão do for pode ser omitida, inclusive fazendo um for infinito assim:
	for(;;) {
		printf(":)");
	}

	// Perceba também que break e continue podem ser usados com qualquer repetição
	
	// Ainda é possível fazer um do-while:
	int x = 0;
	do {
		x++;
	} while(x < 5);
	
	return(0);
}
