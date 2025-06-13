use aws_config::{Region, SdkConfig};
use aws_sdk_s3::{
    config::{Credentials, SharedCredentialsProvider},
    presigning::PresigningConfig,
    types::{
        BucketLifecycleConfiguration, LifecycleExpiration, LifecycleRule, LifecycleRuleFilter,
        Object, Tag,
    },
    Client,
};
use serde::Serialize;
use std::{env::current_exe, error::Error, fs::read_to_string, time::Duration};

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Conf {
    #[serde(rename = "accessKey")]
    pub access_key: String,
    #[serde(rename = "secretKey")]
    pub secret_key: String,
    pub region: String,
    pub bucket: String,
}

impl Conf {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        // 读取 当前项目文件下的 aws.json 文件
        let conf_path = current_exe()?
            .parent()
            .ok_or("Failed to get parent directory")?
            .join("aws.json");

        let conf_str = read_to_string(conf_path)?;

        // 解析json
        serde_json::from_str(&conf_str).map_err(|e| Box::new(e) as Box<dyn Error>)
    }
}

impl From<&Conf> for Credentials {
    fn from(value: &Conf) -> Self {
        let Conf {
            access_key,
            secret_key,
            ..
        } = value;

        Credentials::new(access_key, secret_key, None, None, "aws-vocespace")
    }
}

#[derive(Clone)]
pub struct S3Manager {
    pub client: Client,
    pub conf: Conf,
    pub sdk_conf: SdkConfig,
}

impl S3Manager {
    /// 创建新的 S3 管理器实例
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let conf = Conf::load()?;

        let credential: Credentials = (&conf).into();

        let sdk_conf = SdkConfig::builder()
            .region(Region::new(conf.region.to_string()))
            .credentials_provider(SharedCredentialsProvider::new(credential))
            .build();

        Ok(S3Manager {
            client: Client::new(&sdk_conf),
            conf,
            sdk_conf,
        })
    }

    pub async fn test_connection(&self) -> Result<(), aws_sdk_s3::Error> {
        self.client
            .head_bucket()
            .bucket(&self.conf.bucket)
            .send()
            .await?;

        Ok(())
    }

    pub async fn list_all_objects(&self, prefix: Option<String>) -> Result<Vec<ObjectMetadata>, aws_sdk_s3::Error> {
        let mut objects = Vec::new();
        let mut continuation_token: Option<String> = None;

        loop {
            let mut request = self.client.list_objects_v2().bucket(&self.conf.bucket);

            if let Some(token) = continuation_token {
                request = request.continuation_token(token);
            }

            let response = request.send().await?;

            for object in response.contents() {
                if object.key().is_some() {
                    let mut push = true;
                    // 如果有前缀，则检查对象的键是否以该前缀开头
                    if let Some(prefix) = prefix.as_ref() {
                        if !object.key().unwrap().starts_with(prefix) {
                            push = false;
                        }
                    }

                    if push {
                        objects.push(object.into());
                    }
                }
            }

            // 检查是否有更多对象
            if response.is_truncated().unwrap_or(false) {
                continuation_token = response.next_continuation_token().map(|s| s.to_string());
            } else {
                break;
            }
        }

        Ok(objects)
    }

    /// 删除指定文件
    pub async fn delete_object(&self, key: &str) -> Result<(), aws_sdk_s3::Error> {
        self.client
            .delete_object()
            .bucket(&self.conf.bucket)
            .key(key)
            .send()
            .await?;
        Ok(())
    }

    pub async fn generate_download_url(
        &self,
        key: &str,
        during: u64,
    ) -> Result<String, Box<dyn Error>> {
        let during = Duration::from_secs(during);
        let presigning_config = PresigningConfig::expires_in(during)?;

        let presigned_request = self
            .client
            .get_object()
            .bucket(&self.conf.bucket)
            .key(key)
            .presigned(presigning_config)
            .await?;

        Ok(presigned_request.uri().to_string())
    }

    /// 检查对象是否存在
    pub async fn object_exists(&self, key: &str) -> Result<bool, Box<dyn Error>> {
        match self
            .client
            .head_object()
            .bucket(&self.conf.bucket)
            .key(key)
            .send()
            .await
        {
            Ok(_) => Ok(true),
            Err(err) => {
                // 如果是 NotFound 错误，返回 false，否则传播错误
                if err.to_string().contains("NotFound") {
                    Ok(false)
                } else {
                    Err(Box::new(err))
                }
            }
        }
    }

    /// ## 删除的生命周期规则
    /// 设置对象的删除策略，每个对象只存储3天，超过3天自动删除
    /// 需要筛选tag为`vocespace_record=true`的对象
    pub async fn set_delete_policy(&self) -> Result<(), aws_sdk_s3::Error> {
        let tag = Tag::builder()
            .key("vocespace_record")
            .value("true")
            .build()?;

        let rule = LifecycleRule::builder()
            .id("auto-delete_3_days")
            .status(aws_sdk_s3::types::ExpirationStatus::Enabled)
            .filter(LifecycleRuleFilter::builder().tag(tag).build())
            .expiration(LifecycleExpiration::builder().days(3).build())
            .build()?;

        // 创建生命周期配置
        let lifecycle_config = BucketLifecycleConfiguration::builder()
            .rules(rule)
            .build()?;

        if let Some(exist_lifecycle_conf) = self
            .client
            .put_bucket_lifecycle_configuration()
            .bucket(&self.conf.bucket)
            .get_lifecycle_configuration()
        {
            // 如果已经存在了当前的删除策略，则不需要重复设置
            if exist_lifecycle_conf
                .rules()
                .iter()
                .any(|r| r.id() == Some("auto-delete_3_days"))
            {
                return Ok(());
            } else {
                self.client
                    .put_bucket_lifecycle_configuration()
                    .bucket(&self.conf.bucket)
                    .lifecycle_configuration(lifecycle_config)
                    .send()
                    .await?;
            }
        }

        Ok(())
    }
}

/// 对象元数据结构
#[derive(Debug, Clone, Serialize)]
pub struct ObjectMetadata {
    pub key: String,
    pub size: i64,
    pub last_modified: Option<i64>,
}

impl From<&Object> for ObjectMetadata {
    fn from(value: &Object) -> Self {
        ObjectMetadata {
            key: value.key().expect("Object key must be exist!").to_string(),
            size: value.size().unwrap_or_default(),
            last_modified: value.last_modified().map(|dt| dt.secs()),
        }
    }
}
