geral.html : geral.md
	lowdown -s geral.md -t html -o geral.html

.PHONY : clean test
test : geral.html
	$$BROWSER geral.html
clean :
	-rm geral.html
