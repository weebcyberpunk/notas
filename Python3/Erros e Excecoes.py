# Python3 - Exceções e Erros

# Erros são erros, que sempre acontecem
# Exceções são erros que nem sempre acontecem. Ex: tentar dividir x/0 irá resultar numa exceção, pois é possível dividir x, exceto por 0

# Para tratar erros, faça a estrutura:
    try:
        # tentar fazer
        x/y
    except:
        # se der errado
        print(“deu errado”)
    else:
        # opcional, se der certo
        print(“deu certo”)
    finally:
        # opcional, faz dando certo ou errado
        print(“volte sempre!”)

# Inclusive, é possível tratar o erro usando:
    except Exception as x:
        print(f"problema foi {x}")

# É possível ainda usar x. para especificar a causa, classe, etc, do erro. Ex: x.__class__

# Aliás, cada try pode ter vários except para vários tipo de Exception:
    try:
        x/y
    except (ValueError, TypeError):
        print("problema de tipo")
    except ZeroDivisionError:
        print("divisão por zero impossível")
    except KeyboardInterrupt:
        print("dados não informados")
