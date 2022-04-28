#include<stdio.h>

/*
 * Assim como foi visto em manipulacao_de_bits(), eventualmente é
 * preciso ou melhor guardar informações literalmente bit a bit.
 *
 *
 * Para se criar uma tabela que guarde informações como argumentos
 * passados ao programa bit a bit, primeiro normalmente criam-se
 * constantes que mapeiam para o bit das opções, isso é, o número cujo
 * único bit positivo é a posição em si (ou seja, os números são sempre
 * potências de 2, exceto o 1):
 */
#define SHOW_ALL 1
#define NUMBER_NBLK 2
#define SHOW_ENDS 4
#define NUMBER 8
#define SQUEEZE_BLK 16
#define SHOW_TABS 32
#define SHOW_NONPRINT 64

// ou também:

enum { SHOW_ALL = 1, NUMBER_NBLK = 2, SHOW_ENDS = 4, NUMBER = 8, SQUEEZE_BLK = 16, SHOW_TABS = 32, SHOW_NONPRINT = 64 };

/*
 * Uma maneira prática de se criar a tabela é com uma struct que, com o
 * operador :, mapeia sequências com números específicos de bits em um
 * inteiro:
 */

struct args {
	unsigned int show_all : 1;
	unsigned int number_nblk : 1;
	unsigned int show_ends : 1;
	unsigned int number : 1;
	unsigned int squeeze_blk : 1;
	unsigned int show_tabs : 1;
	unsigned int show_nonprint : 1;
}

/*
 * Nesse caso, todos os bit-fields possuem 1 bit de tamanho. Acessá-los é como
 * acessar valores normais de uma estrutura, porém eles não podem ter ponteiros
 * apontando para eles
 */
