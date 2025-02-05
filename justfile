default:
    echo 'Hello, world!'

sparta: 
    cd ./sparta && cargo run

paper: 
    cd paper && pdflatex main.tex && zathura main.pdf

