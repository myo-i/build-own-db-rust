//リーフノードのヘッダ
pub struct Header{
    //次（図中では右）のページID
    prev_page_id:PageId,
    //前（図中では左）のページID
    next_page_id:PageId,
}

