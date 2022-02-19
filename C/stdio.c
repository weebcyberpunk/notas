#include<stdio.h>

int main() {
  /*
   * Pra começar, precisamos entender que só é possível printar strings. Para printar outras coisas, precisamos usar conversões.
   *
   * Exemplo: para printar somente um número:
   */
  
  printf("%i", 42);
  
  /*
   * Agora, vamos ver como conseguir inputs:
   *
   * Para pegar inputs, precisamos usar o scanf()
   * Novamente, só podemos mexer com strings:
   */
  
  int number;
  scanf("%i", &number); // perceba que é preciso colocar o & para que o scanf altere o valor de number
  
  /*
   * Conversores:
   * 
   * i = int
   * f = float/double
   * o = octadecimal
   * x = hexadecimal
   * c = char
   * s = string
   *
   * Conversores numéricos:
   * e - notação científica
   * g - o compilador decide
   *
   * Perceba que colocar um número entre o % e a letra irá alterar o comportamento da conversão:
   */
  printf("%2i", 100); // printa 100 com no mínimo 2 caracteres de tamanho
  printf("%.2", 1.2345); // printa float com só 2 caracteres após o .

  /*
   * Caracteres especiais:
   *
   * \n - quebra de linha
   * \t - tab
   * \b - backspace
   * \" - aspas duplas
   * \\ - contrabarra
   */
  
  return(0);
}
