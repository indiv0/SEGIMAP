all:

clean:
	rm -rf *.aux *.dvi *.fdb_latexmk *.fls *.log
	rm -rf target/

pdf:
	pdflatex --shell-escape outline.tex
