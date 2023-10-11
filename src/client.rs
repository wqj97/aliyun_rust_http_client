use std::collections::HashMap;

use base64;
use base64::{engine::general_purpose, Engine as _};
use ring::hmac;

fn hmac_sha1(source: &str, secret: &str) -> String {
    let key = hmac::Key::new(hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY, secret.as_bytes());
    let signed_bytes = hmac::sign(&key, source.as_bytes());
    general_purpose::STANDARD.encode(signed_bytes)
}

pub struct AliyunQuery {
    params: HashMap<String, String>,
}


impl AliyunQuery {
    pub fn add_param(&mut self, key: &str, value: &str) {
        self.params.insert(key.to_string(), value.to_string());
    }
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
        }
    }
}

pub struct AliyunClient<'a> {
    access_key_id: &'a str,
    access_key_secret: &'a str,
    aliyun_url: &'a str,
}

impl<'a> AliyunClient<'a> {
    pub fn new(access_key_id: &'a str, access_key_secret: &'a str, aliyun_url: &'a str) -> Self {
        Self {
            access_key_id,
            access_key_secret,
            aliyun_url,
        }
    }
    /// 生成阿里云请求url
    pub fn generate_aliyun_url(self, query: &mut AliyunQuery) -> String {
        let signature_nonce = uuid::Uuid::new_v4().to_string();
        let timestamp = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        let signature_method = "HMAC-SHA1";
        let signature_version = "1.0";
        let version = "2015-01-09";
        query.add_param("AccessKeyId", self.access_key_id);
        query.add_param("RegionId", "cn-hangzhou");
        query.add_param("SignatureType", "");
        query.add_param("Format", "JSON");
        query.add_param("SignatureMethod", signature_method);
        query.add_param("SignatureNonce", &signature_nonce);
        query.add_param("SignatureVersion", signature_version);
        query.add_param("Timestamp", &timestamp);
        query.add_param("Version", version);

        let mut param_map = query.params.iter().collect::<Vec<_>>();
        param_map.sort_by(|a, b| a.0.cmp(&b.0));
        let param_map = param_map
            .iter()
            .map(|s| format!("{}={}", s.0, s.1))
            .collect::<Vec<_>>();
        let raw_param = param_map.clone().join("&");
        let encoded_param = param_map
            .iter()
            .map(|s| {
                let result = url_escape::encode_component(s);
                result
                    .replace("+", "%20")
                    .replace("*", "%2A")
                    .replace("%7E", "~")
                    .replace("%3A", "%253A")
            })
            .collect::<Vec<_>>();
        let encoded_param = encoded_param.join("%26");
        let string_to_sign = format!("GET&%2F&{}", encoded_param);
        let signature = hmac_sha1(
            &string_to_sign,
            &(self.access_key_secret.to_owned() + "&"),
        );
        let encoded_signature =
            percent_encoding::utf8_percent_encode(&signature, percent_encoding::NON_ALPHANUMERIC);

        let final_url = format!(
            "{}/?{}&Signature={}",
            self.aliyun_url, raw_param, encoded_signature
        );
        final_url
    }
}

