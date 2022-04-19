# GNU Make

Make é um programa de automação de builds. Ele lê um arquivo chamado _Makefile_, que especifica como o programa deve atuar para fazer a build de um projeto.

## Regras

Um _Makefile_ simples consiste de simples regras que parecem com o seguinte:

	target ... : prerequisites
		recipe
		...
		...

O "alvo" (target) geralmente é o nome do arquivo gerado pela build, porém também pode ser uma ação (como "clean").  
Os "pré-requisitos" (prerequisites) são os arquivos necessários para se criar o alvo (geralmente o código fonte, podendo necessitar de um ou mais arquivos).  
A "receita" (recipe) é _literalmente_ a receita para se obter o _target_. São os comandos necessários para fazer a build. É necessária estar indentada.

# Receitas

Receitas são os comandos necessários para se buildar um _target_. O make realiza a primeira linha em uma shell, e, caso essa retorne 0, realiza a segunda em uma nova shell. Por padrão, o make irá parar caso uma linha retorne erro. Para ignorar qualquer erro, passa-se a bandeira -i. Para ignorar o erro de uma linha específica, adiciona-se - nela:

	clean :
		# ignora o erro caso a.out já tenha sido apagado
		-rm a.out

# Rodando

Caso rode somente `make`, ele irá buildar o primeiro alvo definido no arquivo. Para escolher outro alvo, basta rodar `make nome-do-alvo`.

Make somente rodará a build caso um ou mais arquivos de pré-requisito sejam mais recentes que o próprio alvo, ou se ele não existir. Caso um alvo tenha pré-requisito em outro alvo, ele verifica a necessidade de buildar o próprio pré-requisito, e o faz caso necessário.

# Simplificando

Make aceita váriaveis de texto. Ex:

	cfiles = main.c foo.c bar.c

	a.out : $(cfiles)
		cc $(cfiles)

Dessa forma, é possível especificar vários arquivos somente uma vez, evitando confusão.

Ainda é possível usar váriaveis de ambiente com ele. Digamos que queremos que um alvo `test` compile um arquivo de Markdown (se necessário) e então abra no browser definido na váriavel $BROWSER, assim como o Makefile desse próprio arquivo é feito:

	geral.html : geral.md
		lowdown -s geral.md -t html -o geral.html

	.PHONY : test
	test : geral.html
		$$BROWSER geral.html

Como o make usa $ internamente (em suas próprias váriveis), para passar o $ para a shell, é necessário escapá-lo com outro $.

# Phony Targets

Phony são alvos que não geram arquivos, como clean. Para evitar que o make se confunda, é de boa prática especificar quais são:

	.PHONY : clean
	clean :
		-rm a.out

# Pré-requisitos

Make possui dois tipos de pré-requisitos:  
- os normais, que simplesmente declaram que o alvo deve ser rebuildado caso sejam mais novos que ele e que serão rebuildados antes disso caso necessário.  
- os `order-only`, cujas builds serão chamadas caso necessárias, porém não forçarão que o próprio alvo seja rebuildado.  

Extremamente útil para criar diretórios que abrigarão os binários, evitando que o binário inteiro seja rebuildado porque o diretório teve uma alteração que mudou seu timestamp.

Para declarar pré-requisitos `order-only`, coloca-se após um |:

	OBJDIR := objdir
		OBJS := $(addprefix $(OBJDIR)/,foo.o bar.o baz.o)

	$(OBJDIR)/%.o : %.c
		$(COMPILE.c) $(OUTPUT_OPTION) $<

	all: $(OBJS)

	$(OBJS): | $(OBJDIR)

	$(OBJDIR):
		mkdir $(OBJDIR)
