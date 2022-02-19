# Python3 - Manipulação de Arquivos

# Para abrir arquivos, use:
    arquivo = open("arquivo", "modo")

# LEMBRE-SE DE FECHÁ-LOS!!!!!!!!
    arquivo.close()

# É possível usar as linhas de arquivos como parâmetros do for

# Há diversos modos de uso:
    "r" # leitura "read"
    "w" # escrita "write". Substitui o conteúdo do arquivo existente
    "x" # escrita. Retorna erro caso o arquivo já exista
    "a" # escrita. Insere dados no final do arquivo. Cria arquivo
    "b" # binário "binary"
    "t" # texto "text". Padrão
    "+" # atualizar, leitura ou escrita

# Comandos:
    open()
        # depois de variável, abre o arquivo no parâmetro, do modo no parâmetro

# Comandos 2:
    .write()
        # escreve o parâmetro no arquivo antes do ponto
    .writelines()
        # recebe um objeto composto e escreve no arquivo
    .close()
        # fecha o arquivo antes do ponto
    .read()
        # para ler o arquivo todo
    .readlines()
        # ler o arquivo linha por linha em uma lista
