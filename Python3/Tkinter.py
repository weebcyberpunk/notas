# Python3 - Tkinter

# A primeira coisa, A PRIMEIRA COISA A SE FAZER QUANDO CRIAR INTERFACES GRÁFICAS É CRIAR A JANELA PRINCIPAL

# Criar uma janela com Tkinter:
    janela = Tk()

# Manter a janela rodando:
    janela.mainloop()

# O primeiro parâmetro das classes é qual janela elas pertencem:
    texto = Label(janela)

# Para decidir a localização dos objetos na janela:
    objeto.grid(column=x, row=y)


# Comandos 2: 
    .mainloop()
        # para manter a janela aparecendo. Todo código abaixo disso não será executado
    .title()
        # para colocar um título na janela
    .grid()
        # para decidir a localização de um objeto
    .geometry()
        # para decidir a geometria da janela, colocar um texto no parâmetro ("AxB")

# Classes:
    Tk()
        # janela principal
    Label(window, text, font)
        # label de texto
    Button(window, text, command)
        # botão. O command é o comando que será executado quando o botão for apertado (não colocar parênteses)
    CheckButton(window, title, command, font)
        # botão true/false
    Entry()
        # entrada de texto simples. .get() para pegar o texto
