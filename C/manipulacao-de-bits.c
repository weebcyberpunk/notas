#include<stdio.h>

int main() {
	/*
	 * MANIPULAÇÃO DE BITS
	 */

	/*
	 * Eventualmente vamos querer guardar várias informações dentro do mesmo
	 * byte, assim como funcionam as flags de permissões de arquivos nos
	 * sistemas Unix. Para isso, precisamos manipular bits em si, com
	 * matemática e lógica binária, e não decimal, onde manipulamos bytes
	 * inteiros.
	 */

	/*
	 * OPERADORES
	 *
	 * Perceba que eles só podem ser usados em dados que sejam operandos
	 * integrais, ou seja, números inteiros (short, int e long) e carácteres
	 * (char)
	 *
	 * & 	AND
	 * | 	inclusive OR
	 * ^ 	exclusive OR
	 * << 	left shift
	 * >> 	right shift
	 * ~ 	complemento (unário)
	 */

	/*
	 * & 	AND
	 *
	 * Positiva todos os bits que forem positivos tanto no primeiro quanto
	 * no segundo termo
	 */

	// usar 0b para binário não é possível na maioria dos compiladores!!!!!!
	n =     0b11011010;
	n = n & 0b01001001;
	//    n é 01001000
	
	/*
	 * | 	OR
	 *
	 * Positiva os bits que são positivos no segundo termo
	 */
	n =     0b01001001
	n = n | 0b11010100
	//    n é 11011101
	
	/*
	 * ^ 	OR exclusivo
	 *
	 * Positiva os bits diferentes e negativa os iguais
	 */
	n =     0b10010110
	n = n ^ 0b01011010
	//    n é 11001100
	
	/*
	 * << >> SHIFT
	 *
	 * Os operadores de shift movem o primeiro termo certo número de bits
	 * para a esquerda ou direita, sendo o número o segundo termo.
	 *
	 * Com unsigned, ele sempre irá preencher os bits restantes com zeros,
	 * porém com signed o comportamento varia de máquina para máquina, com
	 * algumas fazendo o mesmo e outras fazendo o "shift aritmético", que
	 * considera os sinais.
	 */
	n =      0b00000100 // 4
	n = n << 2
	//   n é 0b00010000, aka 16
	// perceba como isso é multiplicar por potências de 2
	
	/*
	 * ~ 	COMPLEMENTO
	 *
	 * Inverte o mundo: troca os 1 por 0 e os 0 por 1
	 */
	n =  0b11001100
	n = ~n
	// n é 00110011
