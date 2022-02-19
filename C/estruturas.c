#include<stdio.h>

int main() {
	// Estruturas são conjuntos de dados que podem ser tratados como uma unidade só
	//
	// Por exemplo, para criar uma estrutura "ponto", que possui uma coordenada x e uma y, fazemos assim:
	struct ponto {
		int x;
		int y;
	};
	// Perceba que uma estrutura não necessariamente precisa de nome (chamado tag)
	// Váriaveis em estruturas são chamadas de membros
	// Membros e tags podem ter o mesmo nome de váriaveis comuns sem problema pois são sempre distinguíveis por contexto
	// Duas estruturas também podem ter membros de nomes iguais
	//
	// Arrisco dizer que estruturas são o protótipo de classes. Estruturas são instanciadas dessa forma:
	struct ponto pt;
	// E podem ser também inicializadas:
	struct pt = {4, 2};

	// Para se referir a um membro de uma estrutura, usa-se o ponto:
	printf("%d", pt.x);

	// Estruturas também podem ser aninhadas:
	struct retangulo {
		struct ponto pt1;
		struct ponto pt2;
	};

	// Um ponto importante: estruturas NÃO DEVEM ser comparadas
	
	// Geralmente, ao usar estruturas grandes que devem ser passadas a funções, é mais fácil usar pointers para elas
	// Lembre-se porém de que pointers irão permitir o acesso a estrutura em si, e não a uma cópia
	// Perceba também que pointer são igualmente usados em estruturas como em váriaveis comuns
	// E se precisar se referir a um membro de uma estrutura indica por pointer:
	*(pt).x;
	// Pretty simple
	//
	// Agora, se pp é pointer para uma estrutura, então o operador -> irá se referir a um membro:
	struct ponto *pp = &pt;
	pp->x;

	// Perceba que ->, assim como () e [] estão no topo da hierarquia de precedência em C
	// Ou seja,
	++pp->x;
	// Irá incrementar o valor de x, pois ++ está se referindo a pp->x;
