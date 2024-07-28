// 封装响应数据 带数据
#[derive(serde::Serialize)]
pub struct ResData<T>
where
    T: serde::Serialize,
{
    pub code: u32,
    pub msg: String,
    pub data: T,
}

// 封装响应数据 不带data
#[derive(serde::Serialize)]
pub struct ResDataNoData {
    pub code: u32,
    pub msg: String,
}
