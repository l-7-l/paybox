use serde::{Deserialize, Serialize};

use super::SinoIdKind;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UBO {
    /// 证件类型
    /// 请填写受益人的证件类型。
    /// 枚举值：
    /// 可选取值：
    /// IDENTIFICATION_TYPE_MAINLAND_IDCARD: 中国大陆居民-身份证
    /// IDENTIFICATION_TYPE_OVERSEA_PASSPORT: 其他国家或地区居民-护照
    /// IDENTIFICATION_TYPE_HONGKONG: 中国香港居民--来往内地通行证
    /// IDENTIFICATION_TYPE_MACAO: 中国澳门居民--来往内地通行证
    /// IDENTIFICATION_TYPE_TAIWAN: 中国台湾居民--来往大陆通行证
    /// IDENTIFICATION_TYPE_FOREIGN_RESIDENT: 外国人居留证
    /// IDENTIFICATION_TYPE_HONGKONG_MACAO_RESIDENT: 港澳居民证
    /// IDENTIFICATION_TYPE_TAIWAN_RESIDENT: 台湾居民证
    pub ubo_id_doc_type: Option<SinoIdKind>,
    /// 证件正面照片
    /// 1、请上传受益人证件的正面照片。
    /// 2、若证件类型为身份证，请上传人像面照片。
    /// 3、可上传1张图片，请填写通过图片上传API预先上传图片生成好的MediaID。
    /// 4、请上传彩色照片or彩色扫描件or复印件（需加盖公章鲜章），可添加“微信支付”相关水印（如微信支付认证）。
    pub ubo_id_doc_copy: Option<String>,
    /// 证件反面照片
    /// 1、请上传受益人证件的反面照片。
    /// 2、若证件类型为护照，无需上传反面照片。
    /// 3、可上传1张图片，请填写通过图片上传API预先上传图片生成好的MediaID。
    /// 4、请上传彩色照片or彩色扫描件or复印件（需加盖公章鲜章），可添加“微信支付”相关水印（如微信支付认证）。
    pub ubo_id_doc_copy_back: Option<String>,
    /// 证件姓名
    /// 该字段需进行加密处理，加密方法详见敏感信息加密说明。(提醒：必须在HTTP头中上送Wechatpay-Serial)
    pub ubo_id_doc_name: Option<String>,
    /// 证件号码
    /// 该字段需进行加密处理，加密方法详见敏感信息加密说明。(提醒：必须在HTTP头中上送Wechatpay-Serial)
    pub ubo_id_doc_number: Option<String>,
    /// 证件居住地址
    /// 1、请按照证件上住址填写，若证件上无住址则按照实际住址填写，如广东省深圳市南山区xx路xx号xx室。
    /// 2、 该字段需进行加密处理，加密方法详见敏感信息加密说明。(提醒：必须在HTTP头中上送Wechatpay-Serial)
    pub ubo_id_doc_address: Option<String>,
    /// 证件有效期开始时间
    /// 1、日期格式应满足合法的YYYY-MM-DD格式
    /// 2、开始时间不能小于1900-01-01
    /// 3、开始时间不能大于等于当前日期
    pub ubo_id_doc_period_begin: Option<String>,
    /// 证件有效期结束时间
    /// 1、日期格式应满足合法的YYYY-MM-DD格式或长期
    /// 2、结束时间大于开始时间
    pub ubo_id_doc_period_end: Option<String>,
}
