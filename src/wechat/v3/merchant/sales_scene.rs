use serde::{Deserialize, Serialize};

// 【店铺信息】 请填写店铺信息
#[derive(Debug, Serialize, Deserialize)]
pub struct SalesSceneInfo {
    /// 【店铺名称】 请填写店铺全称。
    pub store_name: String,
    /// 【店铺链接】
    ///  1、店铺二维码or店铺链接二选一必填。
    ///  2、请填写店铺主页链接，需符合网站规范。
    pub store_url: Option<String>,

    // 【店铺二维码】
    /// 1、店铺二维码 or 店铺链接二选一必填。
    /// 2、若为电商小程序，可上传店铺页面的小程序二维码。
    /// 3、请填写通过图片上传API预先上传图片生成好的MediaID，仅能上传1张图片 。
    pub store_qr_code: Option<String>,

    /// 【商家小程序APPID】
    /// 1、商户自定义字段，可填写已认证的小程序AppID，认证主体需与二级商户主体一致；
    /// 2、完成入驻后， 系统发起二级商户号与该AppID的绑定（即配置为sub_appid，可在发起支付时传入）
    pub mini_program_sub_appid: Option<String>,
}
