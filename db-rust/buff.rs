pub type Page = [u8; PAGE_SIZE];

// #[derive(Debug)]
pub struct Buffer {
    pub page_id: PageId,
    pub page: RefCell<Page>,
    pub is_dirty: Cell<bool>,
}
// #[derive(Debug, Default)]
pub struct Frame {
    usage_count: u64,
    buffer: Rc<Buffer>,
}

// 捨てるバッファを決定し、そのIDを返すメソッド
fn evict(&mutself)->Option<BufferId>{
    let pool_size=self.size();
    let mutconsecutive_pinned=0;
    // 捨てるべきバッファを巡回するためのループ
    // usage_countが0になったら解放している
    let victim_id=loop{
        let next_victim_id=self.next_victim_id;
        let frame=&mutself[next_victim_id];
        // usage_countが0になったら解放
        if frame.usage_count==0{
            //❷
            break self.next_victim_id;
        }
        //貸し出し中かを判定し、そうでなければデクリメント
        if Rc::get_mut(&mutframe.buffer).is_some(){
            //❹
            frame.usage_count=1;
            consecutive_pinned=0;
        }else{
            // 貸し出し中であればインクリメントし、カウンタがプールサイズと同じになったら全てのバッファが貸し出し中ということになる。
            consecutive_pinned+=1;
            if consecutive_pinned>=pool_size{
                return None;
            }
        }
        self.next_victim_id=self.increment_id(self.next_victim_id);
    };
    Some(victim_id)
}


// ページの貸し出し
fn fetch_page(&mutself,page_id:PageId)->Result<Rc<Buffer>,Error> {
    // 欲しいページがバッファプールにある場合
    if let Some(&buffer_id)=self.page_table.get(&page_id){
        let frame=&mutself.pool[buffer_id];
        frame.usage_count+=1;
        return Ok(frame.buffer.clone()
    );

    // 欲しいページがバッファプールにない場合
    //❶evictを呼び出して捨てるバッファ、つまり次に格納するバッファIDを取得
    let buffer_id=self.pool.evict().ok_or(Error::NoFreeBuffer)?;
    let frame=&mutself.pool[buffer_id];
    let evict_page_id=frame.buffer.page_id;
    {
        let buffer=Rc::get_mut(&mutframe.buffer).unwrap();
        //❷is_dirtyはバッファの内容が変更されていて、ディスクの内容が古くなっている事を示すフラグ
        if buffer.is_dirty.get(){
            self.disk.write_page_data(evict_page_id,buffer.page.get_mut())?;
        }
        buffer.page_id=page_id;
        buffer.is_dirty.set(false);
        //❸ページを読み出す
        self.disk.read_page_data(page_id,buffer.page.get_mut())?;
        frame.usage_count=1;
    }
    let page=Rc::clone(&frame.buffer);
    //❹ページテーブルの更新
    self.page_table.remove(&evict_page_id);self.page_table.insert(page_id,buffer_id);

}


fn increment_id(&self,buffer_id:BufferId)->BufferId {
        BufferId((buffer_id.0+1)%self.size())
}

