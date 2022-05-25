# Memória

## Células

Células de memória são feitas de transistores. Um transistor é um dispositivo
como um interruptor: recebe uma informação (0 ou 1) e mantém ela até que ela
seja trocada. O transistor recebe uma voltagem e a mantém. No caso, com
computadores, a voltagem do 1 é sempre a mesma, e a voltagem do zero é não
existir voltagem. Ou seja, quando não há eletricidade correndo, é 0.

Um transistor mantém ou 1 ou 0. Isso é um bit. Um transistor mantém um bit, que
pode ser 1 ou 0. O bit é como o átomo da informação, indivisível.

O transistor precisa de energia constante para manter a informação guardada.

Os transistores possuem um canal de input (para mudar o bit) e um canal de
output (para ver se o bit é 1 ou 0).

## RAM

A memória RAM (memória de acesso randômico) agrupa várias (muitas mesmo) células
de memória, e você pode consultar e alterar uma célula de memória sem afetar as
outras, além de poder ir direto ao ponto da memória que deseja. Discos
magnéticos, por exemplo, são diferentes: precisam rodar até o ponto que deseja
consultar para poder realizar a consulta.

Cada transistor (bit) é numerado, possuindo um endereço na memória. Dessa forma,
sabendo o endereço, é possível ir diretamente até o bit desejado, ao invés de
precisar procurar na memória toda.

A RAM possui pinos de endereçamento, onde é colocado o endereço desejado
a acessar. Também possui um pino de seleção, para escolher se o acesso será de
leitura ou escrita, e um pino de dados, que mostra o valor do bit acessado.

Geralmente cada chip da RAM possui somente um pino de dado, de forma que, para
acessar um byte, teria de se acessar um bit por vez, oito vezes. Para acessar
simultaneamente, mais de um chip de RAM armazena o ***MESMO*** byte, tendo os
bits no mesmo endereço. Dessa forma, acessar o mesmo endereço em oito chips
diferentes lhe trará um byte.

Isso pode variar entre computadores, é claro.

## Organização

Um grupo de 8 bits é um byte, que é a estrutura mais importante que tem. Além
disso, um grupo de dois bytes é uma _word_ (palavra), 4 bytes é uma _double
word_ (palavra dupla) e 8 bytes é uma _quad word_ (palavra quartúpla).

Computadores costumam acessar a memória em chunks de 4 ou 8 bytes (32 ou 64
bits). Mesmo assim, cada byte tem seu próprio endereço na memória. Os antigos
computadores de 8 bits acessavam a memória apenas um byte por vez.

Ou seja, computadores de 32 bits acessam a memória consultando os 4 bytes
a partir do endereço fornecido, que é o endereço do primeiro byte consultado.
Pode-se ignorar os próximos 3 bytes, porém teria de se acessar novamente
a memória para consultá-los.

## Acesso

A CPU faz o trabalho de acessar a memória, lendo ou escrevendo. Porém, boa parte
do trabalho é deixado para os periféricos, como a própria RAM, GPU, etc.

Mais sobre a CPU e seu acesso a memória é tratado em `cpu.md`.
