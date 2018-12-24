Rust練習用リポジトリ
================================================================================

### 作成中 or 作成予定

- 2018-06-17 ゆゆゆいステータスプロット (yuyuyui\_status)
- 2018-06-17 家計簿、資産管理 (未)

### 作成済

- 2017-01-22 cat

### Install
--------------------------------------------------------------------------------
### [The Rust toolchain installer](https://rustup.rs/)

```
curl https://sh.rustup.rs -sSf | sh
```

### Package Manager
```
# pacman -S rustup
$ rustup toolchain install stable
$ rustup default stable
```

### Environment
--------------------------------------------------------------------------------
### rust.vim, rustfmt
> This is a Vim plugin that provides Rust file detection, syntax highlighting, formatting, Syntastic integration, and more.

`rust.vim`を入れると`rustfmt.vim`も一緒についてくる。
`/usr/bin/rustfmt`はArchLinuxのパッケージでインストールしたけど、`cargo install`でもできるらしい。

- [rust-lang/rust.vim: Vim configuration for Rust.](https://github.com/rust-lang/rust.vim)

### racer
> _RACER_ = _R_ust _A_uto-_C_omplete-_er_.
> A utility intended to provide Rust code completion for editors and IDEs.
> Maybe one day the 'er' bit will be exploring + refactoring or something.

`/usr/bin/racer`はArchLinuxのパッケージでインストールしたが、`cargo install`でもできる（上と同じ）。

- [racer-rust/racer: Rust Code Completion utility](https://github.com/racer-rust/racer)
- [racer-rust/vim: Racer support for vim](https://github.com/racer-rust/vim-racer)

### vimrc

```
if dein#load_state(expand('~/.vim/dein'))
  call dein#begin(expand('~/.vim/dein'))
  ...
  call dein#add('rust-lang/rust.vim')
  call dein#add('racer-rust/vim-racer')
  ...
  call dein#end()
  call dein#save_state()
endif
...
let g:rustfmt_autosave = 1
set hidden

au FileType rust nmap gd <Plug>(rust-def)
au FileType rust nmap gs <Plug>(rust-def-split)
au FileType rust nmap gx <Plug>(rust-def-vertical)
au FileType rust nmap <leader>gd <Plug>(rust-doc)
```

### Command

- `<C-x><C-o>`：コード補完
- `???`：タグジャンプ
- `???`：タグジャンプ（水平分割）
- `???`：タグジャンプ（垂直分割）
- `???`：ドキュメントを開く

### 参考
--------------------------------------------------------------------------------

- [Rust Documentation](https://doc.rust-lang.org/)
    - [Rust Programming Language](https://doc.rust-lang.org/book/)
    - [The Rust Reference](https://doc.rust-lang.org/reference.html)
    - [Standard Library API Reference](https://doc.rust-lang.org/std/)
    - [The Rustonomicon](https://doc.rust-lang.org/nomicon/)
    - [Compiler Error Index](https://doc.rust-lang.org/error-index.html)
- [The Rust Programming Language](https://doc.rust-jp.rs/book/second-edition/)
- [Rust初心者向けハンズオン](https://chikoski.github.io/rust-handson/)
- [Cargo : packages for Rust](https://crates.io/)
