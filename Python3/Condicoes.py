# Python3 - Condições (if)

# Uma condição simples:
    if condição:
        bloco

# Uma condição composta:
    if condição:
        # bloco
    else:
        # bloco

# Mais de uma condição:
    if condição:
        # bloco
    elif outra condição:
        # bloco
    elif mais uma condição:
        # bloco
    else:
        # bloco

# Pretty simple, don't?

# Condição simplificada:
    # Vou usar exemplo:
        print("novo" if idade <= 60 else "velho")

    # da pra usar o mesmo if para várias condições usando um or ou and, exemplo:
        if nome == "GG" or nome == "Hayase":
            # bloco
        elif nome == "GG" and idade == "17":
            # bloco
