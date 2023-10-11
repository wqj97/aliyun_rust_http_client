//! 这是一个阿里云的Rust SDK, 用于访问阿里云的各种服务.
//! 仅处理了阿里云的签名算法, 输入参数后, 会返回一个签名后的URL.
//! 用任意的HTTP客户端访问这个URL, 即可访问阿里云的服务.
//!
//! ```
//! use aliyun_rust_http_client::client::{AliyunClient, AliyunQuery};
//!
//! let client = AliyunClient::new(
//!     "access_key_id",
//!     "access_key_secret",
//!     "https://xxx.aliyuncs.com"
//! );
//! let mut query = AliyunQuery::new();
//! query.add_param("Action", "DescribeDomainRecords");
//! let url = client.generate_aliyun_url(&mut query);
//! reqwest::get(&url);
//! ```
pub mod client;