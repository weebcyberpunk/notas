# Python3 - Listas

# Listas também são variáveis compostas

# Listas são escritas entre colchetes

# Listas podem ser vazias

# Diferente das tuplas, listas podem ter um elemento trocado:
    sobremesa = ["pudim", "paçoca"]
    sobremesa[1] = "sorvete"
    print(sobremesa)

        # Resulta em:
            ["pudim", "sorvete"]

# tentar apagar de uma lista um elemento inexistente resulta em erro, porém facilmente contornável assim:
    if "elemento" in lista:
        lista.remove("elemento")

# IMPORTANTE: ao atribuir o valor a uma variável como uma outra lista, assim:
    x = y

    # qualquer alteração feita em qualquer uma das listas também vai ser feita na outra. Para evitar isso, faça assim:
        x = y[:]

# da pra colocar listas dentro de listas:
    x.append(y[.])
# quando tiver uma lista dentro de uma lista, use isso pra "no item x, o item y":
    lista[x][y]

# Comandos:
    del
    # apaga a lista ou o elemento da lista da memória, ex: del sobremesa[0]
    list()
    # quando declarar a variável, use list para declarar como lista
    len()
    # tamanho da lista

# Comandos 2:
    .append()
        # após uma lista, adiciona o que está entre parênteses ao final da lista
    .insert(x, y)
        # adiciona y a lista antes do ponto, porém na posição x, movendo todos os elementos necessários (ex: se adicionar algo a posição 1, o 0 continua igual, porém o 1 se torna 2, o 2 se torna 3, etc)
    .pop()
        # apaga o elemento dentro do parêntese da lista antes do ponto. Sem especificar o elemento, ele apaga o último. Move os elementos necessários
    .remove()
        # igual o pop porém ao invés de indicar o índice, índica o valor, ex: sobremesa.remove("paçoca"). Move os elementos necessários, elimina o primeiro elemento (da pra eliminar todos com laço)
    range(x, y)
        # usado com o list(), cria uma lista na contagem entre x e y excluindo y. Funciona igual o range do for
    .sort()
        # depois da lista, coloca a variável organizada em ordem crescente. Usando (reverse=True) faz ordem decrescente 
    .clear()
        # apaga todos os elementos da lista, porém sem apagar a lista
    tuple()
        # converte a lista dentro do parêntese em tupla
