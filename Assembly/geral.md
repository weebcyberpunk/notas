# Geral

Dados e funções são tratados e chamados especificando o endereço onde eles começam.

O program counter sempre aponta para a próxima instrução e é constantemente incrementado, exceto que o programa deva voltar instruções ou ir a outras funções.

O mais importante de Assembly é entender os endereços de memória, já que sempre estamos lidando com ele e não com algo abstrato como váriaveis.

## A linha de produção

O tempo todo há comunicação entre a CPU e os periféricos. Exemplo: certa data chega via um cabo conectado à porta de rede. O primeiro byte é colocado no _bus_, a CPU o processa e coloca no _bus_ para a GPU o colocar na memória de vídeo para que você consiga enxergar a informação na tela.

E quem diz para a CPU e os periféricos fazerem isso tudo? VOCÊ FAZ, O PROGRAMADOR. Você escreve um programa, que é nada mais nada menos que dados na memória.

Assim como existem certos códigos em binário que significam coisas para nós (os números em si, os caractéres, etc), a CPU também possui códigos que ela entende, que são as ***instruções de máquina***, que são essencialmente instruções para com a CPU. Quando a CPU está realizando coisas, ela pega sequências de números do _bus_, cada um dizendo para ela fazer uma coisa. Ao completar uma instrução, ela passa para a próxima, e continua executando até que algo mande parar (um comando no programa ou um sinal como o botão de reset).

Mais sobre instruções de máquina será tratado em `instrucoes.md`.
