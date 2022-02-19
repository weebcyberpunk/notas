# Classes são tipos de objetos que podem receber parâmetros e métodos personalizados

# No nosso exemplo, teremos uma classe Computador que terá uma marca, uma quantidade de RAM e uma placa de vídeo

# Classes são declaradas da seguinte forma:
    class Computador:
        def __init__(self):
            self.marca = "Asus"
            self.ram = "8gb"
            self.placa = "Nvidia"
        pass

    # Classes sempre começam com letra maíscula


# Para realmente usar uma classe, deve-se declarar uma variável com ela:
    computador1 = Computador()

# A função __init__ serve para definir os parâmetros para a varíavel. Sendo assim, dessa forma:
    print(computador1.marca)

    # Resulta em:
        Asus

# Para declarar classes com parâmetros personalizados:

    class Computador:
        def __init__(self, marca, ram, placa):
            self.marca = marca
            self.ram = ram
            self.placa = placa
        pass
        # Dessa forma, será necessário passar os parâmetros ao declarar:
            computador1 = Computador(marca="Asus", ram="8gb", placa="Nvidia")

# Para criar métodos para a classe:

    class Computador:
        def __init__(self, marca, ram, placa):
            self.marca = marca
            self.ram = ram
            self.placa = placa
        def ligarComputador(self):
            print(f"estou ligando")
        pass

        # Dessa forma:
            computador1 = Computador("Asus", "8gb", "Nvidia")
            computador1.ligarComputador()

            # Resulta em:
                estou ligando
