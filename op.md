git add .
git commit -am "Update"
git push

git remote add origin https://gitlab.eduxiji.net/pku2200013170/compiler_lab.git
git branch -M main
git push -uf origin main


autotest -koopa -s lv1 /root/compiler/compiler_lab

cargo run -- -koopa hello.c -o hello.koopa
