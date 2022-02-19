# Python3 - Módulos

# Cria-se um módulo fazendo um arquivo .py e importando ele no programa principal

# Pacotes são conjuntos de módulos. Cria-se um pacote criando uma pasta e importando no programa principal. Dentro de cada pacote, precisa-se de um arquivo __init__.py

# Recomenda-se criar os módulos como pastas com o __init__.py neles, esse init funcionando como o arquivo com as def, e essas pastas dentro dos pacotes

# Alguns módulos úteis:

math
    # Comandos:
        ceil()
            # arredonda pra cima
        floor()
            # arredonda pra baixo
        trunc()
            # tira o decimal
        sqrt()
            # raiz quadrada
        factorial()
            # calcula fatorial
            
decimal
    # Módulo que melhora a matemática de ponto flutuante
    # O módulo aplica um contexto de aritmética para o programa, onde é definida a precisão de ponto flutuante, as traps (DivisionByZero e etc) e outras questões.
    # Números decimais no módulo são definidos com a classe Decimal

random
    # Comandos
        choice()
            # escolhe randomicamente um número da lista dentro do parêntese (colocar lista com [])
        randint(x, y)
            # escolhe randomicamente um número inteiro entre x e y, incluindo x e y (x e y podem ser substituídos por qualquer número inteiro). Para botar em tuplas:
                x = (randint(), randint(), randint())
                # No número de vezes necessárias

datetime
    # Objetos:
        date
            # uma data em A-D-M. Exemplo: 2021-01-11
        time
            # uma hora em H:M:S.MS Exemplo: 17:27:18.973703
        datetime
            # uma data com hora
        timedelta
            # uma diferença de tempo

        # objetos date possuem year, month e day
        # objetos time possuem hour, minute, second e microsecond
        # objetos datetime possuem todos acima
        # objetos timedelta possuem days, seconds, microseconds, milliseconds, minutes, hours e weeks. Porém somente days, seconds e microseconds são armazenados 
        # fazer operações de - entre dois datetime/date/time geram um timedelta

    # Para definir uma datetime sem ser hoje:
        variável = datetime(year, month, day, hour, minute, second, microsecond)

    # Comandos 2:
        .today()
            # usar da seguinte maneira:
                variavel = objeto.today()
                # ele define a variável como sendo o dia de hoje
        .now()
            # mesma coisa que .today() para ser usado com datetime e time

time
    # Comandos:
        sleep()
            # espera a quantidade de segundos no parêntese para continuar a rodar

playsound # instalado com pip
    playsound()
        # toca o arquivo de áudio entre os parênteses (nome como string). Aceita MP3 e WAVE

webbrowser and requests # requests é instalado com pip
    open()
        # do webbrowser, abre a página no navegador padrão 
    get()
        # do requests, usando declarando variável, a variável se tornará a response da request feita ao site no parâmetro. Da erro se não for possível acessar o site

threading
    Thread(função)
        # ao declarar uma váriavel (x), coloca a função numa thread x
    .start()
        # inicia a thread antes do ponto
