# Registradores

Registradores são os bolsos e a mesa de trabalho da CPU, como foi explicado em `cpu.md`.

Aliás, periféricos também tem registradores, geralmente mais específicos e confusos que os da CPU, porém geralmente a conversa com periféricos é feita através do sistema operacional.

## Os registradores em si

### Especiais

- _instruction pointer_: o ponteiro de instrução aponta para a próxima instrução na memória e é atualizado a cada instrução realizada pela CPU.  
- _flags_: registradores de 1 bit que indicam coisas a CPU.  
