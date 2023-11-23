# ctp-sys
以 Rust 的方式，简化对接 CTPAPI 的过程，节省精力，快速上手

## 简介
ctp-sys 提供的 ctp 官方 ctpapi(c++) 的rust版本， 使用 bindgen 转换 ctpapi(c++) 生成。

ctp-sys 目前支持 6.6.5.x(linux)版本， 对应 ctpapi(c++) 的生产版本: v6.6.5_20210924

通过 ctp-sys 库只能连接支持 ctpapi(c++) 官方实现的柜台，如：simnow; 不支持连接所谓的兼容 ctpapi(c++) 接口.

# 问题记录
1. 使用相对路径时，wrapper.h 中头文件需要使用双引号，否则会报错`Unable to generate bindings: ClangDiagnostic("wrapper.h:1:10: error: 'ctp/6.6.7_20220304/linux64/tradeapi_se/ThostFtdcUserApiStruct.h' file not found with <angled> include; use \"quotes\" instead\n")`
2. linux库名需要lib+"库名称"软连接后才能被ld读取找到
3. c++的头文件需要使用wrapper.hpp引入，否则无法识别class关键字：[Generating Bindings to C++](https://rust-lang.github.io/rust-bindgen/cpp.html)
4. bindgen 不支持c++的虚拟函数，使用[rust-share](https://github.com/mineralres/rust-share)方案。


# 相关资源
1. [API来源：上期技术官方网站](http://www.sfit.com.cn/5_2_DocumentDown_2.htm)
2. [tradeapi v6.6.5_20210924](http://www.sfit.com.cn/DocumentDown/api_3/5_2_2/v6.6.5_tradeapi.zip)
3. [请参看官方文档](http://www.sfit.com.cn/DocumentDown/api_3/5_2_2/6.6.5_APIInterfacedescription_0301.zip)

# 致谢
1. [OPENCTP](https://github.com/openctp/openctp/tree/master/6.6.9_20220920)
2. [rust-share](https://github.com/mineralres/rust-share)
