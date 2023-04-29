use std::convert::TryInto;
use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*, SeekFrom};
use std::path::Path;

use zerocopy::{AsBytes, FromBytes};

pub struct DiskManager{
  //ヒープファイルのファイルディスクリプタ
  heap_file:File,

  // 採番するページIDを決めるカウンタ(64ビット整数)
  // ページIDの実体はただの整数値ですが、専用の構造体を定義することでほかの整数値と区別しています。
  // こうすることで、ページID同士の足し算などの無意味な演算を型検査でエラーにでき、バグを作りづらくなります。
  next_page_id:u64,
}

pub const PAGE_SIZE: usize = 4096;

pub struct PageId(pub u64);

impl DiskManager{
  //コンストラクタ
  pub fn new(data_file:File)>io::Result<Self>{
    //ファイルサイズを取得
    let heap_file_size = heap_file.metadata()?.len();
    let next_page_id = heap_file_size/PAGE_SIZE as u64;
    Ok(Self{
      heap_file,
      next_page_id,
    })
  }

  //ファイルパスを指定して開く（io::Resultが例外時の返り値、<Self>が成功時の返り値）
  pub fn open(data_file_path:impl AsRef<Path>) ->io::Result<Self>{
    let heap_file = OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(heap_file_path)?;// ?は返り値がエラーの場合早期リターン
    Self::new(heap_file) // DiskManager::new()メソッドを呼び出している
  }

  // 第一引数に&mutや&mut selfが指定されている場合、それらはインスタンスメソッド（インスタンスしか使えない）となる
  // 無い場合はスタティックメソッド
  
  // 新しいページIDを採番する
  // 新規のファイルを作成だが、やっていることはページIDを採番しているだけ
  pub fn allocate_page(&mut self)>PageId{
    let page_id = self.next_page_id;
    self.next_page_id += 1;//採番するたびにインクリメント
    PageId(page_id)  
  }

  //ページのデータを読み出す
  pub fn read_page_data(&mut self,page_id:PageId,data:&mut[u8]) ->io::Result<()>{
    //オフセットを計算
    let offset = PAGE_SIZE as u64 * page_id.to_u64();
    
    //ページ先頭へシーク
    self.heap_file.seek(SeekFrom::Start(offset))?;
    
    //データを読み出す
    self.heap_file.read_exact(data)
  }

  //データをページに書き出す
  pub fn write_page_data(&mut self,page_id:PageId,data:&[u8]) ->io::Result<()>{
    //オフセットを計算（ブロックごとに書き込みや読み込みを行うため、ページサイズ*ページID）
    let offset = PAGE_SIZE as u64 * page_id.to_u64();

    //ページ先頭へシーク
    self.heap_file.seek(SeekFrom::Start(offset))?;

    //データを書き込む
    self.heap_file.write_all(data)
  }
}



fn main() {
  // 世界よ、こんにちは
  println!("Hello, world!");
  println!("Hello, KAWAMEN!!");
}