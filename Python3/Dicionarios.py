# Python3 - Dicionários

# Dicionários são variáveis compostas semelhantes às tuplas e listas, porém eles podem ter índices literais e personalizados, e não necessariamente os índices numéricos padrões

# Dicionários são mutáveis como as listas

# Dicionários são escritos entre { }

# Para declarar um dicionário com índice personalizado:
    comida = {"entrada": "batata", "prato": "estrogonofe"}

# O print com índices personalizados é normal:
    print(comida["entrada"])

# Para adicionar novos dados, NÃO usar o append, fazer assim:
    comida["sobremesa"] = "pudim"

# É possível ainda criar um dicionário pulando linhas:
    comida = {"entrada": "batata",
              "prato": "estrogonofe",
              }

# Dá pra usar os valores, índices e itens do dicionário junto com o laço for:
    for x, y in comida.items():

# Para encontrar valores de dicionários pelo índice dentro de listas:
    pessoas = [{"nome": "gg"}]
    print(pessoas[0]["gg"])

# Para usar f-string:
    >>> pessoa = {"nome": "gg", "idade": 16}
    >>> print(f"oi {pessoa['nome']}"
    oi gg

# IMPORTANTE: SE USAR " NA STRING, USAR ' NO ÍNDICE E VICE-VERSA

# Para deixar um dicionário na ordem:
    from operator import itemgetter
    resultado = {3, 1, 2}
    rank = sorted(resultado.items(), key=itemgetter(x))
# não pula linha

# Sendo x 0 ou 1, se for 0, ele ordena pelo índice, se for 1, ordena pelo valor
    # pra fazer do maior ao menor, usar reverse=True após a key:
        rank = sorted(resultado.items(), key=itemgetter(x), reverse=True)

# QUANDO COLOCA UM DICIONÁRIO EM ORDEM, A VARIÁVEL SE TORNA UMA LISTA COM TUPLAS DENTRO

# É POSSÍVEL USAR fstring COM DICIONÁRIOS:
    >>> sobremesa = {“c1”: “pudim”}
    >>> x = "1"
    >>> sobremesa[f"c{x}"] = "paçoca"
    >>> print(sobremesa["c1"])
    paçoca


# Comandos:
    dict()
        # usa para declarar uma variável como um dicionário
    del x[y]
        # elimina índice y do dicionário x

# Comandos 2:
    .values()
        # retorna todos os valores do dicionário
    .keys()
        # retorna todos os índices (chaves) do dicionário
    .items()
        # retorna tudo do dicionário
    .copy()
        # copia um dicionário
