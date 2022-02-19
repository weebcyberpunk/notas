#include<stdio.h>
#include<stdbool.h>

int main() {
  // Para criar uma váriavel booleana sem libs, usa-se:
  _Bool ggIsCool;
  
  // Lembre-se que em C, booleanos são 0 e 1 (true pode ser qualquer número, false deve ser 0), e não true e false
  ggIsCool = 1;
  
  
  // Outra alternativa é usando a stdbool.h, e então o formato a ser usado será mais conveniente:
  bool ggIsCool = true;
  
  return(0);
}
