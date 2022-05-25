# Execução dos Programas

Em x85 (32 bits), primeiro a CPU pega (_fetch_) os primeiros 4 bytes de um
endereço da memória. Essa _double word_ é lida da memória e então carregada na
CPU. A CPU processa essa primeira _double word_ e então retorna a memória para
carregar a próxima instrução. Dentro da CPU há um registrador especial chamado
_instruction pointer_ (ponteiro de instrução) que aponta sempre para a próxima
instrução na memória. O ponteiro é atualizado sempre que a CPU começa uma nova
instrução.

Todo esse processo é realizado de maneira sincronizada com o _clock_,
literalmente o relógio do computador. O _clock_ pulsa em intervalos precisos,
e todos os transistores da CPU se coordenam com ele. CPUs antigas levavam alguns
ciclos para realizar uma instrução, porém CPUs modernas conseguem realizar
processos em paralelo, então mais de um processo pode ser feito no mesmo ciclo
do _clock_.

Perceba que o _instruction pointer_ pode ser modificado "on demand", para mudar
qual será a próxima instrução a ser realizada. O programa não necessariamente
segue uma ordem linear, ele pode voltar ou pular instruções.
