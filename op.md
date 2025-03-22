git add .
git commit -am "Update"
git push

autotest -koopa -s lv1 /root/compiler/compiler_lab

cargo run -- -koopa hello.c -o hello.koopa
