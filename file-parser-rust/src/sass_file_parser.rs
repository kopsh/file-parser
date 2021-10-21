use csv::Reader;
use std::error::Error;

pub struct SassFileParser {
    path: String,
}

impl SassFileParser {
    pub fn new(path: String) -> SassFileParser {
        SassFileParser { path: path }
    }

    /*
    交易订单号,原交易订单号,通道参考号,清算主键,交易商户名称,交易商户编号,交易类型,系统交易日期,
    系统交易时间,卡号,业务结算方式,金额,卡类型,卡组织,所属银行,交易状态,银行通道名称,通道商户编号,
    系统错误码,系统错误描述,银行错误码,代理商名称,交易来源,代理商交易商户编号,手续费,结算金额,
    商家实收金额,优惠券金额,是否手续费后付,是否手工退款,通道交易状态
     */
    pub fn parse(&self) -> Result<(), Box<dyn Error>> {
        let mut rdr = Reader::from_path(&self.path)?;
        for result in rdr.records() {
            let record = result.unwrap();
            println!("{:?}", record);
            break;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::SassFileParser;

    #[test]
    fn test_parse() {
        let parser =
            SassFileParser::new("/Users/songhao/Documents/baobao/sass_total.csv".to_string());
        assert_eq!(parser.parse().unwrap(), ());
    }
}
