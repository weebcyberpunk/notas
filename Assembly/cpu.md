# CPU

A CPU (unidade central de processamento) é a principal parte do computador.

## Geral

Assim como a memória, a CPU também possui pinos de dados e endereços, que são conectados a RAM. Para ler a memória, a CPU coloca o endereço nos pinos de endereço, e a RAM retorna os valores em seus pinos de dados, que são copiados pela CPU para seus próprios pinos. Para escrever, a CPU coloca novamente o endereço nos pinos de endereço e o conteúdo nos pinos de dados.

# Registradores

A CPU possui um conjunto de pontos de armazenamento de dados chamados ***registradores***. Esses registradores são o lugar onde a CPU vai armazenar dados rapidamente. É possível usar a memória para isso, mas acessar a memória é lento. Os registradores são de rápido acesso pois estão dentro da própria CPU.

Os registradores são os bolsos e mesa de trabalho da CPU. Não só para armazenar rapidamente (bolsos), mas também para trabalhar. Por exemplo: se a CPU precisa somar dois números, ela pode simplesmente colocar cada um em um registrador, somá-los e então substituir um deles pela soma, ou ainda colocar a soma em outro registrador, ou ainda somar com outro número em outro registrador, etc.

A questão é que, os registradores são o _armazenamento para os trabalhos que a CPU está fazendo no momento_.

Trabalhar com os registradores é sempre rápido, porque eles estão bem integrados uns com os outros e com as partes internas da CPU.

Assim como tudo, registradores são feitos de transistores, mas ao invés de terem endereços, possuem nomes.

Para deixar mais complicado, mesmo que em geral os registradores tenham características em comum, alguns são especiais.

Falo mais deles em `registradores.md`.
