# Python3 - Laços: geral e for

# Laços são estruturas usadas para fazer os comandos se repetirem definida ou indefinidamente

# O laço for tem um número limitado de repetições definido

# O controle é o comando que definirá quando o laço vai acabar 

# Laços devem ter um nome adquirido, e são escritos assim:
    for nome in controle:
        # comandos do laço
    # comando após laço terminar

# se você fizer o laço print(nome do laço), ele vai mostrar quantas vezes o laço se repete (range). Da pra usar isso com o algoritmo normal de aritmética dentro do parêntese do range
# podemos fazer um acumulador dessa forma:
    soma = 0
    for c in range(x, y):
        soma = soma+c

        # dessa forma, a soma final será a soma de todos os números de 1 a 10
        # da mesma forma, se substituirmos o último c por 1, ele irá contar quantas vezes o programa rodou, e passa a se chamar contador

# Se usar "for c in tupla:" (ou lista), c vai receber cada elemento da tupla e a repetição ocorrerá de acordo com a quantidade de elementos da tupla

# Para usar o for com tuplas usando tanto o elemento quanto a posição do elemento, precisa se usar duas variáveis, exemplo:
    sobremesa = "pudim", "paçoca"
    for x, y in enumerate(sobremesa):
        print(x, y)

        # Resulta em:
            0 pudim
            1 paçoca

# Dá pra usar os valores, índices e itens do dicionário junto com o laço for:
    for x, y in comida.items():

# Comandos 2:
    range(x, y)
        # faz o laço se repetir entre x e y, que podem ser substituídos pelo número de vezes que quer que o laço se repita. Exemplo: para fazer o laço se repetir 6 vezes, coloca range(0, 6)
    enumerate()
        # usado com duas variáveis e uma tupla dentro do parêntese, a primeira variável se torna a posição do elemento da tupla

