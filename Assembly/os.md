# Sistema Operacional

O OS (Operational System) é um programa como qualquer outro, com a diferença que ele serve para gerenciar os recursos de hardware do computador e fornecer uma interface para o usuário.

Sistemas operacionais de micro-computadores antigos, como o CP/M, eram extremamente simples: basicamente, um terminal onde se podia digitar o nome de um programa. O programa era lançado pelo sistema operacional e _substituia_ o sistema na memória do computador. Basicamente, todos os recursos do computador eram entregues ao programa, e quando esse era fechado, o sistema operacional era rebootado.

Com o passar do tempo e o embarateamento da memória, sistemas operacionais começaram a permitir que mais de um programa fosse rodado ao mesmo tempo e não eram apagados da RAM quando tivessem outros programas rodando.

## BIOS

O PC DOS, da IBM, vinha com um módulo especial para manter o sistema ativo enquanto outro programa estivesse na RAM: um chip de memória somente de leitura (ROM) que mantém os dados estando com ou sem energia, fazia o gerenciamento dos dispositivos de input e output do computador. Esse sistema foi chamado de BIOS (Basic Input/Output System).

Atualmente, softwares como a BIOS, que estão no hardware de maneira "não-volátil", são chamados de _firmware_.

## Multitask (multitarefa)

As primeiras versões do Windows nada mais eram que gerenciadores de arquivos e lançadores de programas arranjados em gráficos, porém com basicamente o DOS rodando por baixo. O Windows 95 criou então o **modo protegido** (que ele na verdade não usava 100%), que fazia com que o sistema operacional tivesse total controle sobre o hardware, enquanto outros programas não conseguiam fazer o mesmo.

Além disso, o Windows 95 implementou o multitasking, onde o sistema operacional dá o controle da CPU para um programa, algumas de suas instruções são rodadas, então o sistema passa o controle a outro programa que também está carregado na RAM, que também roda algumas instruções, e o controle da CPU é passado a outro programa, etc. Como cada programa toma uma fração de segundo para rodar suas instruções, e então o programa atualmente em execução muda, a impressão é de que há vários programas rodando **AO MESMO TEMPO**.

Em 1991, o Kernel Linux havia sido desenvolvido como kernel para sistemas Unix-like e já tinha capacidade multitarefa. A principal diferença desse último para o Windows 95 era de que o Linux utilizava completamente o modo protegido. A memória era separada entre o espaço do kernel (_kernel space_) e o espaço do usuário (_user space_). Nada rodando no _user space_ pode escrever (e geralmente sequer ler) o que estiver rodando no _kernel space_. A comunicação entre os dois é feita através de syscalls (ver mais em `syscalls.md`) estritamente controladas. Além disso, a interface do usuário é completamente separada do sistema operacional em si, fazendo com que o kernel ficasse protegido de problemas causados por mal-funcionamento de outros programas. O acesso direto a periféricos e qualquer hardware é limitado a programas rodando no _kernel space_, e programas rodando no _user space_ só podem acessá-los via drivers carregados no kernel.

Eventualmente, a Microsoft percebeu como o jeito Unix de fazer as coisas era daora e aplicou a ideia de separação estrita entre _kernel_ e _user space_ em seus novos sistemas desde o Windows 2000.

### Cores (Núcleos)

PCs começaram a ser distribuídos com mais de uma CPU por volta de 2000. Tanto o Windows quanto o Linux tem suporte a mais de uma CPU. No geral, quando mais de uma CPU está disponível, o OS roda seu próprio código em uma e roda os programas do _user space_ em outra.

Eventualmente entre 2005 e 2006, a Intel e a AMD começaram a distribuir CPUs que, mesmo sendo um só chip, possuiam duas unidades de execução de código idênticas e independentes. Isso é, aí começa a história das CPUs de mais de um núcleo. CPUs de quatro núcleos começaram a virar realidade mais tarde, e no período em que escrevo, a AMD está trabalhando em uma CPU com 108 núcleos.
