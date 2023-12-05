use std::{fs, io::Write, path::Path};

use ctp_sys::{
    print_rsp_info, set_cstr_from_str_truncate_i8, CThostFtdcReqUserLoginField, CtpAccountConfig, gb18030_cstr_to_str, trading_day_from_ctp_trading_day, ascii_cstr_to_str, ascii_cstr_to_str_i8,
};
use futures::StreamExt;
use tracing::info;

#[tokio::main]
async fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    // 初始化日志
    tracing_subscriber::fmt::init();

    let account = CtpAccountConfig {
        broker_id: "9999".to_string(),
        account: "-".to_string(),
        trade_front: "tcp://180.168.146.187:10201".to_string(),
        // md_front: "tcp://180.168.146.187:10131".to_string(),
        md_front: "tcp://180.168.146.187:10211".to_string(),
        name_server: "".to_string(),
        auth_code: "0000000000000000".to_string(),
        user_product_info: "".to_string(),
        app_id: "simnow_client_test".to_string(),
        password: "-".to_string(),
    };
    md(&account).await;
}

fn check_make_dir(path: &String) {
    // 创建目录
    match fs::create_dir_all(path) {
        Ok(_) => info!("目录创建成功：{}", path),
        Err(e) => info!("无法创建目录：{}->{}", path, e),
    }
}

async fn md(ca: &CtpAccountConfig) {
    use ctp_sys::md_api::*;
    let broker_id = ca.broker_id.as_str();
    let account = ca.account.as_str();
    let md_front = ca.md_front.as_str();
    let auth_code = ca.auth_code.as_str();
    let user_product_info = ca.user_product_info.as_str();
    let app_id = ca.app_id.as_str();
    let password = ca.password.as_str();
    let mut request_id: i32 = 0;
    let mut get_request_id = || {
        request_id += 1;
        request_id
    };
    let flow_path = format!(".cache/ctp_futures_md_flow_{}_{}/", broker_id, account);
    check_make_dir(&flow_path);
    let mut api = create_api(&flow_path, false, false);
    let mut stream = {
        let (stream, pp) = create_spi();
        api.register_spi(pp);
        stream
    };
    use std::ffi::CString;
    api.register_front(CString::new(md_front).unwrap());
    info!("register front {}", md_front);
    api.init();
    // 处理登陆初始化查询
    while let Some(spi_msg) = stream.next().await {
        use ctp_sys::md_api::CThostFtdcMdSpiOutput::*;
        match spi_msg {
            OnFrontConnected(_p) => {
                let mut req = CThostFtdcReqUserLoginField::default();
                set_cstr_from_str_truncate_i8(&mut req.BrokerID, broker_id);
                set_cstr_from_str_truncate_i8(&mut req.UserID, account);
                set_cstr_from_str_truncate_i8(&mut req.Password, password);
                api.req_user_login(&mut req, get_request_id());
                info!("OnFrontConnected");
            }
            OnFrontDisconnected(p) => {
                info!("on front disconnected {:?} 直接Exit ", p);
                std::process::exit(-1);
            }
            OnRspUserLogin(ref p) => {
                info!("OnRspUserLogin");
                if p.p_rsp_info.as_ref().unwrap().ErrorID == 0 {
                    let u = p.p_rsp_user_login.unwrap();
                    let mut v = vec![];
                    let mut d = chrono::Local::now();
                    for i in 0..12 {
                        d = d.checked_add_months(chrono::Months::new(1)).unwrap();
                        let symbol = std::ffi::CString::new(format!(
                            "ru{}",
                            d.format("%Y%m").to_string().trim_start_matches("20")
                        ))
                        .unwrap();
                        v.push(symbol);
                    }
                    let count = v.len() as i32;
                    info!("symbols={:?}", v);
                    let ret = api.subscribe_market_data(v, count);
                    info!("Subscribe ret = {}", ret);
                } else {
                    info!("Trade RspUserLogin = {:?}", print_rsp_info!(&p.p_rsp_info));
                }
            }
            OnRtnDepthMarketData(ref md) => {
                info!("md={:?}", md);

                if let Some(dmd) = md.p_depth_market_data{
                    info!("交易日期: TradingDay:{:?}", trading_day_from_ctp_trading_day(&dmd.TradingDay));
                    info!("品种（保留字段）: reserve1:{:?}", ascii_cstr_to_str_i8(&dmd.reserve1));
                    info!("交易所代码: ExchangeID::{:?}", ascii_cstr_to_str_i8(&dmd.ExchangeID));
                    info!("空（保留字段）: reserve2::{:?}", ascii_cstr_to_str_i8(&dmd.reserve2));
                    info!("最新价: LastPrice::{:?}", &dmd.LastPrice);
                    info!("上次结算价: PreSettlementPrice:::{:?}", &dmd.PreSettlementPrice);
                    info!("昨收盘: PreClosePrice:::{:?}", &dmd.PreClosePrice);
                    info!("昨持仓量: PreOpenInterest:::{:?}", &dmd.PreOpenInterest);
                    info!("今开盘: OpenPrice:::{:?}", &dmd.OpenPrice);
                    info!("最高价: HighestPrice:::{:?}", &dmd.HighestPrice);
                    info!("最低价: LowestPrice:::{:?}", &dmd.LowestPrice);
                    info!("数量: Volume:::{:?}", &dmd.Volume);
                    info!("成交金额: Turnover:::{:?}", &dmd.Turnover);
                    info!("持仓量: OpenInterest:::{:?}", &dmd.OpenInterest);
                    info!("今收盘: ClosePrice:::{:?}", &dmd.ClosePrice);
                    info!("本次结算价: SettlementPrice::::{:?}", &dmd.SettlementPrice);
                    info!("涨停板价: UpperLimitPrice:::{:?}", &dmd.UpperLimitPrice);
                    info!("跌停板价: LowerLimitPrice:::{:?}", &dmd.LowerLimitPrice);
                    info!("昨虚实度: PreDelta:::{:?}", &dmd.PreDelta);
                    info!("今虚实度: CurrDelta:::{:?}", &dmd.CurrDelta);
                    info!("最后修改时间: UpdateTime:::{:?}", ascii_cstr_to_str_i8(&dmd.UpdateTime));
                    info!("最后修改毫秒: UpdateMillisec:::{:?}", &dmd.UpdateMillisec);
                    info!("当日均价: AveragePrice:::{:?}", &dmd.AveragePrice);
                    info!("业务日期: ActionDay:::{:?}", trading_day_from_ctp_trading_day(&dmd.ActionDay));
                    info!("合约代码: InstrumentID:::{:?}", ascii_cstr_to_str_i8(&dmd.InstrumentID));
                    info!("合约在交易所的代码: ExchangeInstID:::{:?}", ascii_cstr_to_str_i8(&dmd.ExchangeInstID));
                    info!("上带价: BandingUpperPrice:::{:?}", &dmd.BandingUpperPrice);
                    info!("下带价: BandingLowerPrice:::{:?}", &dmd.BandingLowerPrice);

                }
            }
            _ => {}
        }
    }
    api.release();
    api.join();
    // Box::leak(api);

    tokio::time::sleep(std::time::Duration::from_secs(100)).await;
    info!("完成保存查询结果");
}
