#include<stdio.h>
#include<stdlib.h>
#include<curses.h>

/*
 * ncurses é uma biblioteca GNU para criar programas
 * de terminal com interface que imita programas de
 * GUI
 *
 * Para utilizá-la, basta ter instalado a biblioteca,
 * incluir o curses.h e passar a bandeira -lncurses
 * para o compilador
 */

int basico() {

	// A primeira função que deve ser chamada por um
	// programa em ncurses para que a biblioteca inicie
	// suas estruturas de dados e determine
	// características do terminal que está em uso
	initscr();

	// recebe um único caractere qualquer sem echo
	// útil para manter o terminal na janela do ncurses
	// enquanto uma tecla não for apertada
	getch();

	// a função que é chamada para retirar um terminal
	// do controle da biblioteca
	endwin();
	
	return(0);
}
