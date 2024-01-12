use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum NotifiedRefundStatus {
    Success,
    Abnormal,
    Closed,
}

impl NotifiedRefundStatus {
    /// Returns `true` if the notified refund status is [`Abnormal`].
    ///
    /// [`Abnormal`]: NotifiedRefundStatus::Abnormal
    #[must_use]
    pub fn is_abnormal(&self) -> bool {
        matches!(self, Self::Abnormal)
    }

    /// Returns `true` if the notified refund status is [`Closed`].
    ///
    /// [`Closed`]: NotifiedRefundStatus::Closed
    #[must_use]
    pub fn is_closed(&self) -> bool {
        matches!(self, Self::Closed)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RefundNotifySource {
    /// 对开启结果数据进行加密的加密算法，目前只支持AEAD_AES_256_GCM
    pub algorithm: String,
    // 加密前的对象类型，退款通知的类型为refund
    // 示例值：refund
    pub original_type: String,
    /// Base64编码后的开启/停用结果数据密文
    /// 示例值：fdasfsadsadsalkja484w
    pub ciphertext: String,
    // 附加数据
    // 示例值：fdasdsadsafcsflkja484w
    pub associated_data: String,
    /// 加密使用的随机串
    /// 示例值：fdasfjihihihlkja484w
    pub nonce: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RefundNotify {
    pub id: String,
    pub create_time: String,
    pub event_type: NotifiedRefundStatus,
    pub summary: String,
    pub resource_type: String,
    pub resource: RefundNotifySource,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DecodedRefundNotifyAmount {
    /// 订单总金额，单位为分，只能为整数
    pub total: u64,
    /// 退款金额，币种的最小单位，只能为整数，不能超过原订单支付金额，如果有使用券，后台会按比例退。
    pub refund: u64,
    /// 用户实际支付金额，单位为分，只能为整数
    pub payer_total: u64,
    /// 退款给用户的金额，不包含所有优惠券金额
    pub payer_refund: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DecodedRefundNotifySource {
    pub mchid: String,
    #[cfg(feature = "platform")]
    pub sub_mchid: String,
    pub out_trade_no: String,
    pub transaction_id: String,
    pub out_refund_no: String,
    pub refund_id: String,
    /// 退款状态，枚举值：
    /// SUCCESS：退款成功
    /// CLOSED：退款关闭
    /// ABNORMAL：退款异常，退款到银行发现用户的卡作废或者冻结了，导致原路退款银行卡失败，可前往【商户平台—>交易中心】，手动处理此笔退款
    /// 示例值：SUCCESS
    pub refund_status: NotifiedRefundStatus,
    /// 退款成功时间
    pub success_time: Option<String>,
    /// 退款入账账户
    ///     取当前退款单的退款入账方。
    /// 1、退回银行卡：{银行名称}{卡类型}{卡尾号}
    /// 2、退回支付用户零钱: 支付用户零钱
    /// 3、退还商户: 商户基本账户、商户结算银行账户
    /// 4、退回支付用户零钱通：支付用户零钱通
    /// 示例值：招商银行信用卡0403
    pub user_received_account: String,
    pub amount: DecodedRefundNotifyAmount,
}
