use neon::prelude::*;
use std::collections::HashMap;

mod bank_file_parser;
use bank_file_parser::BankFileParser;
mod order;
mod sass_file_parser;
use order::Order;

fn entrance(mut cx: FunctionContext) -> JsResult<JsString> {
    // let bank_file_path = cx.argument::<JsString>(0).unwrap();
    // let sass_file_path = cx.argument::<JsString>(1).unwrap();

    // parse_files(bank_file_path, sass_file_path);
    Ok(cx.string("OK"))
}

fn parse_files(bank_file_path: String, sass_file_path: String) {
    let bank_file_parser = BankFileParser::new(bank_file_path.to_string());
    let reader = bank_file_parser.parse().unwrap();
    let mut i = 0;
    let mut order_map: HashMap<String, Order> = HashMap::new();
    let mut rdr = csv::Reader::from_reader(reader);
    println!("开始处理银行文件...");
    for result in rdr.records() {
        i += 1;
        if i % 1000 == 0 {
            println!("{}", i);
        }
        match result {
            Err(_) => continue,
            Ok(record) => {
                let order_id = record[6].strip_prefix('`').unwrap();
                let paid = record[12]
                    .strip_prefix('`')
                    .unwrap()
                    .parse::<f32>()
                    .unwrap_or(0.0);
                let refund = record[16]
                    .strip_prefix('`')
                    .unwrap()
                    .parse::<f32>()
                    .unwrap_or(0.0);
                let refund_status = record[19].strip_prefix('`').unwrap();
                if refund > 0.0 && (refund_status != "PROCESSING" && refund_status != "SUCCESS") {
                    continue;
                }
                if order_map.contains_key(order_id) {
                    if let Some(order) = order_map.get_mut(order_id) {
                        order.bank_refund += refund;
                    }
                } else {
                    let order = Order {
                        id: order_id.to_string(),
                        bank_fund: paid,
                        bank_refund: refund,
                        sass_fund: 0.0,
                    };
                    order_map.insert(order_id.to_string(), order);
                }
            }
        }
    }

    let mut rdr = csv::Reader::from_path(sass_file_path).unwrap();
    println!("开始处理sass文件...");
    /*
    交易订单号,原交易订单号,通道参考号,清算主键,交易商户名称,交易商户编号,交易类型,系统交易日期,
    系统交易时间,卡号,业务结算方式,金额,卡类型,卡组织,所属银行,15-交易状态,16-银行通道名称,通道商户编号,
    系统错误码,系统错误描述,银行错误码,代理商名称,22-交易来源,代理商交易商户编号,手续费,结算金额,
    26-商家实收金额,优惠券金额,是否手续费后付,是否手工退款,通道交易状态
     */
    for result in rdr.records() {
        match result {
            Err(_) => println!("error from sass"),
            Ok(record) => {
                if &record[16] == "新平台微信支付宝网联通道"
                    && &record[22] == "微信扫码"
                    && &record[15] == "交易成功"
                {
                    let order_id = &record[0];
                    let fund = record[26].parse::<f32>().unwrap_or(0.0);
                    if order_map.contains_key(order_id) {
                        let order = order_map.get_mut(order_id).unwrap();
                        order.sass_fund = fund;
                        if order.picked() {
                            order_map.remove(order_id);
                        }
                    } else {
                        order_map.insert(
                            order_id.to_string(),
                            Order {
                                id: order_id.to_string(),
                                bank_fund: 0.0,
                                bank_refund: 0.0,
                                sass_fund: fund,
                            },
                        );
                    }
                }
            }
        }
    }
    for order in order_map.values() {
        println!("{}", order);
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("parse_files", entrance)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parse_files;

    #[test]
    fn test_parse_files() {
        let bank_file_path =
            String::from("/Users/songhao/Documents/baobao/1505378701All2021-09-21网联.csv");
        let sass_file_path = String::from("/Users/songhao/Documents/baobao/sass_total.csv");

        parse_files(bank_file_path, sass_file_path);
    }
}
