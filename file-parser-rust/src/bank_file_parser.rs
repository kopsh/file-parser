
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use encoding_rs::GBK;
use encoding_rs_io::{DecodeReaderBytesBuilder, DecodeReaderBytes};

pub struct BankFileParser {
    path: String,
}

impl BankFileParser {
    pub fn new(path: String) -> BankFileParser {
        BankFileParser { path: path }
    }

    /*["交易时间", "公众账号ID", "商户号", "特约商户号", "设备号", "网联订单号", 6-"商户订单号", "用户标识",
     "交易类型", "交易状态", "付款银行", "货币种类", 12-"应结订单金额", "代金券金额", "网联退款单号", "商户退款单号"
    , 16-"退款金额", "充值券退款金额", "退款类型", 19-"退款状态", "商品名称", "商户数据包", "手续费", "费率", "订单金额", "申请退款金额", "费率备注"]
    */
    pub fn parse(&self) -> Result<BufReader<DecodeReaderBytes<File, Vec<u8>>>, Box<dyn Error>> {
        let f = File::open(self.path.clone()).unwrap();
        let mut i = 0;
        let reader = BufReader::new(DecodeReaderBytesBuilder::new().encoding(Some(GBK)).build(f));
        Ok(reader)
    }
}

#[cfg(test)]
mod tests {
    use super::BankFileParser;

    #[test]
    fn test_parse() {
        let parser = BankFileParser::new(
            "/Users/songhao/Documents/baobao/1505378701All2021-09-21网联.csv".to_string(),
        );
    }
}
