cargo doc
rsync -avh target/doc/ docs/
ln -s docs/matrix_display/index.html docs/index.html
