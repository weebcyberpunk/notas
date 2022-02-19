#include<stdio.h>

int main() {
	// Pointers são, basicamente, váriaveis que guardam o endereço na memória para outra váriavel
	// O operador & retorna o endereço da váriavel. Dessa forma:
	int n = 1;
	p = &n;
	// p vira um pointer de n
	//
	// * é o operador que permite acessar o endereço a qual o pointer aponta, portanto:
	int x = 1, y = 2, z[10];
	int *ip; // ip é pointer para int
	ip = &x; // ip é pointer para x
	y = *ip; // y é 1, mesmo valor de x
	*ip = 0; // x é zero
	ip = &z[0]; // ip é pointer para z[0]

	// Como funções em C simplesmente copiam os parâmetros, para alterar as váriaveis passadas, deve-se passar pointers para elas. Ao invés de fazer algo como:
	void add(int x) {
		x = x++;
	}
	// Deve-se fazer:
	void add(int *x) {
		*x = *x++;
	}

	return(0);
}
