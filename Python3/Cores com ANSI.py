# Python3 - Cores com ANSI escape sequence

# Usar o \033 porque é o melhor pro Python

# Para fazer um print com o padrão desejado, colocar o \ dentro da string. Exemplo: "\033[1;34;46mOlá, Mundo!". Assim ele irá formatar tudo o que tiver depois dele. Se um novo código for colocado na mesma string, ele re-formatará tudo o que tiver depois desse código

# Para adicionar a cor:
    "\033[m"

# Entre o [ e o m, colocar os códigos de estilo, texto e cor, separado por ;
    # Os códigos podem estar em qualquer ordem e podem aparecer ou não
    # O padrão do terminal é fundo preto, letra cinza e sem formação. Usando \033[m ele gera isso, exceto que o padrão tenha sido alterado
    # Para usar letras pretas, inverta as configurações de um fundo preto normal com a letra

# Códigos:
    # Estilo:
        # 0 normal
        # 1 bold (negrito)
        # 4 sublinhado
        # 7 inverter configuração

    # Texto:
        # 30 branco
        # 31 vermelho
        # 32 verde
        # 33 amarelo
        # 34 azul
        # 35 roxo
        # 36 ciano
        # 37 cinza (cor padrão)

    # Cor: mesmo padrão do texto porém em 40
