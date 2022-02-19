// A primeira coisa a se fazer em um código de C é declarar as bibliotecas que serão usadas. Para isso:

#include<lib>

// Todo programa em C deve ter um método main, portanto:

main() {
  // Agora falando do geral da linguagem
  
  /*
   * C é uma linguagem baseada primariamente em funções (diferente de Java, que é orientada a objetos e baseada em classes)
   * Classes são permitidas em programas C se você usar a linguagem C++, que é, basicamente, C com orientação a objeto
   *
   * Funções em C devem ser declaradas uma por arquivo
   *
   * C é uma linguagem space insensitive, o que significa que os espaços, quebras de linha e tabulações não farão diferença para a compilação (o que não significa que deve-se escrever código feio)
   *
   * C possui dados no padrão ansi:
   *
   * -----------------------------------
   *   tipo  |  tamanho  |   exemplo
   * -----------------------------------
   *   bool  |   1 bit   |     true
   * -----------------------------------
   *   char  |   1 byte  |     'g'
   * -----------------------------------
   *   int   |  2 bytes  |   -32728
   * -----------------------------------
   *  float  |  4 bytes  |  0.000000F
   * -----------------------------------
   *  double |  8 bytes  | 0.0000000000
   * -----------------------------------
   *
   * Váriaveis em C não podem começar por número e nem possuir caracteres especiais. São case-sensitive
   * 
   * Váriaveis tem tipo fixo e são declaradas com =
   *
   * Perceba que char usa '' e strings, que são arrays de char, usam ""
   */
  
  int x;
  x = 5;
  int a = 10;
  
  // para que uma função altere a váriavel de argumento, deve-se colocar & no argumento, EXCETO SE FOR ARRAY:
  
  int number;
  scanf("%i", &number);
  
  // para utilizar typecast, exemplo com divisão float:
  
  double result = (double) x / 2;
  
  return(0);
}
