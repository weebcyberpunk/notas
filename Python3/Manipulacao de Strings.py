# Python3 - Manipulação de Strings

# Anotações gerais:
    # Fatiamento: separação da string por carácteres
    # Análise: informações  

# Strings são listas de letras. Da pra usar elas com for igual se usa as tuplas 

# ao definir uma seção da string com carácteres, ex: (3, 6), o Python considerara o primeiro como incluso porém desconsidera o segundo
# ao usar print(" ") porém usar 3 aspas ao invés de uma de cada lado, ele considera a string com linhas puladas
# ao usar frase[2:4] você está definindo que quer somente a frase entre os carácteres 2 a 4. Ao usar frase[2:4:2], é a mesma coisa pulando de dois em dois carácteres
# para definir o a contagem ao contrário, use números negativos

# Hint: para formatar um print de número float, decidindo o tamanho dele:
    print("{:.1f}".format(1.56123))

# Comandos:
    len()
        # retorna o comprimento da string dentro dos parênteses

# Comandos 2:
    .count()
        # após uma variável string, conta quantas vezes tem o que está dentro dos parênteses nessa string. Se adicionar números (ex: ("o", 0, 13) ele faz a contagem entre esses carácteres
    .find()
        # após uma variável string, mostra em que momento começa a seção da string colocada dentro dos parênteses. Se tiver mais de uma na string, considera a primeira
    .rfind()
        # mesma coisa que o .find só que da esquerda pra direita
    .replace(" ", " ")
        # após uma variável string, substitui o que está antes da vírgula pelo o que está depois dentro dessa string
    .upper()
        # após uma variável string, deixa em maiúsculo a string toda. Precisa dos dois parênteses após, e ele não muda as letras já maiúsculas
    .lower()
        # contrário do .upper()
    .capitalize()
        # após uma variável string, deixa tudo em minúsculo exceto o carácter 0. Precisa dos dois parênteses após
    .title()
        # após uma variável string, deixa o primeiro carácter de cada palavra maiúsculo. Precisa dos dois parênteses após
    .strip()
        # após uma variável string, remove os espaços antes e depois da frase em si. Precisa dos parênteses após
    .rstrip()
        # mesma coisa do .strip() porém remove somente os espaços depois da frase (espaços a direita)
    .lstrip()
        # mesma coisa do .rstrip() só que no início, na esquerda
    .split()
        # após uma variável string, separa ela em mais strings, tornando-a uma lista de strings. Por padrão o carácter separador é o espaço
    " ".join( )
        # junta as strings de uma lista (dentro dos parênteses) usando o carácter entre aspas como a junção
