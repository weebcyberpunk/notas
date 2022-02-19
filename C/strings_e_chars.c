#include<stdio>

int main() {
  // Para criar strings, é necessário criar arrays de char
  // Perceba que arrays de char precisam de um espaço a mais que o total pois a string precisa de um caráctere dizendo que a mesma terminou
  
  char name[31];
  
  // para usar com scanf:
  scanf("%s", name); // perceba que não precisa do & pois é um array
  // lembre-se também que não é possível solicitar coisas com espaço: o computador ignorará tudo depois dos espaços
  
  // Para usar strings com caracteres especiais como \n ou \t. use uma \ antes deles:
  printf("\\t é a tabulação");
  
  return(0);
  
  // Perceba que cada caractere possui um número correspondente. O número de A por exemplo é 65
}
