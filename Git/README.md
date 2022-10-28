## Github配置

生成密钥（设置密钥存储路径及密码）：

```bash
ssh-keygen -t rsa -b 4096 -C "choutsugi@gmail.com"
```

查看公钥（以默认路径为例）：

```bash
cat ~/.ssh/id_rsa.pub
```

Github中添加公钥：

```Text
Github➡️Settings➡️SSH and GPG keys➡️New SSH key➡️粘贴公钥
```

验证：

```bash
ssh -T git@github.com
```

## Git配置

```bash
git config --global user.name tsugi
git config --global user.email choutsugi@gmail.com
git config --global push.default matching
git config --global core.quotepath false
git config --global core.editor "vim"
```

