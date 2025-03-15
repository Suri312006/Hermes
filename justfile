default:
    echo 'Hello, world!'

sparta: 
    cd ./sparta && cargo run --release

paper: 
    cd paper && pdflatex main.tex && zathura main.pdf

