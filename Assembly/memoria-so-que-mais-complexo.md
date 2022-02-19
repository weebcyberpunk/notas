# Memória, porém mais complexo

Traduzindo livremente o terceiro parágrafo da página 77 de _Assembly Step by Step_, de Jeff Dunteman:

> _A ingênua objeção que existe para uma CPU executar instruções de máquina pode ser disposta de maneira simples: ela executa instruções de máquina assim que ela as têm em suas mãos eletrônicas. O_ ***real*** _trabalho de uma CPU, e o real desafio da linguagem Assembly, está em localizar as tais instruções de máquina e dados na memória. Qualquer idiota consegue aprender instruções de máquina (Muitos o fazem.)_ ***A habilidade da linguagem Assembly consiste em uma compreensão profunda de endereçamento de memória.*** _Qualquer coisa a parte disso são detalhes - e detalhes fáceis a esse ponto._

Resumindo, entender o endereçamento da memória é importante pra \*\*\*\*.

## Modelos

Há diferentes modelos de endereçar a memória em x86, porém há três principais. Em x86 32 bits para GNU/Linux, nós vamos focar em um deles, pois os outros estão caindo em desuso.

### Real Mode Flat Model (Modelo Plano de Modo Real)

O modelo mais antigo.

### Real Mode Segmented Model (Modelo Segmentado de Modo Real)

O segundo modelo é provavelmente a coisa que mais dá raiva de estudar em todo o mundo da ciência da computação.

Na época do Intel 8080, o CP/M-80 era o OS mais comum para o mesmo. O 8080 era uma CPU de 8 bits, porém tinha uma saída de endereçamento de 16 bits, o que endereçava 64kb. A época, as máquinas geralmente não passavam dos 4kb a 8kb. O modelo de memória dele fazia o seguinte: o CP/M-80 era carregado no **topo** da memória existente, e os programas eram, quando necessários, carregados a partir do endereço 0100H. Os primeiros 256 bytes sobrando eram chamados de prefixo do segmento de programa (_program segment prefix_) e possuia um punhado de informações e um buffer para o I/O do programa.

Ao criar uma CPU de 16 bits, a 8086, a Intel queria continuar permitindo que programas escritos para CP/M-80 do 8080 funcionassem na 8086. A 8086 podia endereçar até incríveis 1mb (16x a capacidade da 8080), então a Intel fez com que fosse possível um programa rodar inteiro dentro de um chunk de 64kb, que era endereçado atráves de um registrador de segmento (_segment register_).

O problema disso foi que novos programas, feitos para o próprio 8086 e que não precisariam do modelo segmentado precisaram então usar a memória em chunks de 64kb, e se precisassem trocar de segmentos, precisariam trocar os valores nos _segment registers_.

Quando rodando em _real mode segmented model_, processadores x86 podem acessar diretamente até 1mb de memória (100000H bytes), chamada de _real mode memory_. Lembre-se que o último endereço de 100000H bytes é FFFFFH, pois o computador começa a contar do zero.

CPUs de x86 32 bits conseguem endereçar 4gb de memória sem dividi-la em segmentos menores. Quando operando em _protected mode flat model_, o segmento **é** de 4gb. Para ter compatibilidade com software antigo de DOS, as CPUs possuem o chamado _virtual-86 mode_, em que ela se limita a usar somente o que a 8086 conseguia usar. Isso deu certo, de verdade.

Quando operando em _real mode segmented model_, CPUs de x86 32 bits se configuram para endereçar apenas 1mb de memória. Elas usam somente 20 de seus 32 pinos de endereçamento. Porém, além disso, elas ainda se configuram para separar tudo em chunks de 64kb. Literalmente, elas podem "enxergar" 1mb inteiro porém se "vendam" para acessar apenas 64kb.

### Protected Mode Flat Model (Modelo Plano de Modo Protegido)

O modelo mais moderno que está por trás de sistemas operacionais modernos como o GNU/Linux e o Windows. Só funciona a partir das CPUs i386 que suportam a arquitetura IA-32. É bem parecido com o _real mode flat model_.
