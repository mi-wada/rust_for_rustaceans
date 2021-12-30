# Chapter 1
## Core

## Question

## Memo
- rustの基礎を固めるための章
- rustについて知るために、メモリについて解説する
- valueとvariableとpointerの違いについて
  - value
    - ある型とその型の値の領域の組み合わせ
    - どこに格納されているかによって、valueの意味は変わらない
  - variable
    - valueの格納先
    - スタック上の名前付きvalue格納庫
  - pointer
    - メモリ上の住所を示すvalue
- constとは
  - constで宣言した値は，プログラム内の参照箇所でその値に置き換えられる
- スタック
  - 呼び出された関数のローカル変数・引数・戻り値などが置かれる
  - 関数が戻るとスタックは片付けられる
- ヒープ
  - スタックに紐付かないメモリプール
  - RustではBoxを使って扱う

(2021/12/30) 次はBorrowing and Lifetimesから
