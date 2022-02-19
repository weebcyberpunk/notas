# Python3 - Funções 

# Funções são… funções
# Quando você precisa fazer uma mesma coisa várias vezes, transforme-a numa função, assim você só faz isso uma única vez

# Variáveis no programa principal continuam existindo na função, porém variáveis na função só existem na função, não existem no programa principal. Para usar uma variável global dentro da função (para que ela sofra alteração no programa principal também), usar o comando global):
    def funcao()
        global x

# Para fazer uma função é simples:
    def sobremesa():
        print(“pudim”) 

        # Desse jeito, sempre que usar sobremesa(), print(“pudim”) será utilizado

# Para usar parâmetros, para personalizar o próprio def, coloque-os dentro dos parênteses:
    def sobremesa(doce):
        print(doce, “É MUITO BOM”)


    sobremesa(“pudim”)

        # Resulta em:
            pudim É MUITO BOM

# por questão estética, sempre coloque uma quebra de 2 linhas entre os def e o código principal

# Caso precise usar números personalizados de parâmetros, é necessário fazer o empacotamento deles, fazendo com que virem uma tupla:
    def sobremesa(*doce):
        print(doce)


    sobremesa(“pudim”)
    sobremesa(“paçoca”, “sorvete”)

        # Resulta em:
            (“pudim”, )
            (“paçoca”, “sorvete”)

# Caso for usar uma variável composta no parâmetro, não precisa fazer o empacotamento

# Para usar parâmetros opcionais, isso é, parâmetros que podem ou não serem usados:
    def somar(x=0, y=0, z=0):
        s = x + y + z
        print(s)

# Dessa forma, se variáveis não forem aferidos, então eles valerão 0

# Para funções que atribuam valores:
    def somar(x=0, y=0, z=0):
        s = x+y+z
        return s


    r = somar(x, y, z)

        # Desse jeito, r irá receber o valor de s. Pode inclusive fazer sem variáveis:

            def somar(x=0, y=0, z=0):
                s = x+y+z
                return s


            print(somar(x, y, z))

    # Ainda da pra usar variáveis booleanas com isso:

    def par(x):
        if x%2 == 0:
            return True
        else:
            return False


    x = 2
    if par(x):
        print(“par”)
    else:
        print(“ímpar”)

# Docstrings:
    # Para criar um manual para sua função, abra “”” no início da def e escreva o manual, feche “”” e continue o código da função 
