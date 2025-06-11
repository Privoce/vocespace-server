use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client, Error, presigning::PresigningConfig};
use std::time::Duration;

pub struct S3Manager {
    client: Client,
    bucket_name: String,
}

impl S3Manager {
    /// 创建新的 S3 管理器实例
    pub async fn new(bucket_name: String) -> Result<Self, Error> {
        // 从环境变量或默认配置加载区域
        let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
        let config = aws_config::from_env().region(region_provider).load().await;
        let client = Client::new(&config);

        Ok(S3Manager {
            client,
            bucket_name,
        })
    }

    /// 从指定的区域和凭证创建 S3 管理器
    pub async fn new_with_region(bucket_name: String, region: &str) -> Result<Self, Error> {
        let region_provider = RegionProviderChain::first_try(region);
        let config = aws_config::from_env().region(region_provider).load().await;
        let client = Client::new(&config);

        Ok(S3Manager {
            client,
            bucket_name,
        })
    }

    /// 测试连接到 S3 bucket
    pub async fn test_connection(&self) -> Result<(), Error> {
        self.client
            .head_bucket()
            .bucket(&self.bucket_name)
            .send()
            .await?;
        
        println!("Successfully connected to bucket: {}", self.bucket_name);
        Ok(())
    }

    /// 查询 S3 bucket 的所有对象
    pub async fn list_all_objects(&self) -> Result<Vec<String>, Error> {
        let mut objects = Vec::new();
        let mut continuation_token: Option<String> = None;

        loop {
            let mut request = self.client
                .list_objects_v2()
                .bucket(&self.bucket_name);

            if let Some(token) = continuation_token {
                request = request.continuation_token(token);
            }

            let response = request.send().await?;

            if let Some(contents) = response.contents() {
                for object in contents {
                    if let Some(key) = object.key() {
                        objects.push(key.to_string());
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

    /// 查询 S3 bucket 的对象（带前缀过滤）
    pub async fn list_objects_with_prefix(&self, prefix: &str) -> Result<Vec<String>, Error> {
        let mut objects = Vec::new();
        let mut continuation_token: Option<String> = None;

        loop {
            let mut request = self.client
                .list_objects_v2()
                .bucket(&self.bucket_name)
                .prefix(prefix);

            if let Some(token) = continuation_token {
                request = request.continuation_token(token);
            }

            let response = request.send().await?;

            if let Some(contents) = response.contents() {
                for object in contents {
                    if let Some(key) = object.key() {
                        objects.push(key.to_string());
                    }
                }
            }

            if response.is_truncated().unwrap_or(false) {
                continuation_token = response.next_continuation_token().map(|s| s.to_string());
            } else {
                break;
            }
        }

        Ok(objects)
    }

    /// 删除指定文件
    pub async fn delete_object(&self, key: &str) -> Result<(), Error> {
        self.client
            .delete_object()
            .bucket(&self.bucket_name)
            .key(key)
            .send()
            .await?;

        println!("Successfully deleted object: {}", key);
        Ok(())
    }

    /// 批量删除多个文件
    pub async fn delete_objects(&self, keys: Vec<&str>) -> Result<(), Error> {
        use aws_sdk_s3::types::{Delete, ObjectIdentifier};

        let objects: Vec<ObjectIdentifier> = keys
            .into_iter()
            .map(|key| ObjectIdentifier::builder().key(key).build().unwrap())
            .collect();

        let delete = Delete::builder()
            .set_objects(Some(objects))
            .build()
            .unwrap();

        let response = self.client
            .delete_objects()
            .bucket(&self.bucket_name)
            .delete(delete)
            .send()
            .await?;

        if let Some(deleted) = response.deleted() {
            for obj in deleted {
                if let Some(key) = obj.key() {
                    println!("Successfully deleted: {}", key);
                }
            }
        }

        Ok(())
    }

    /// 生成预签名下载链接（15分钟有效期）
    pub async fn generate_download_url(&self, key: &str) -> Result<String, Error> {
        self.generate_download_url_with_expiry(key, Duration::from_secs(15 * 60)).await
    }

    /// 生成指定有效期的预签名下载链接
    pub async fn generate_download_url_with_expiry(&self, key: &str, expires_in: Duration) -> Result<String, Error> {
        let presigning_config = PresigningConfig::expires_in(expires_in)?;

        let presigned_request = self.client
            .get_object()
            .bucket(&self.bucket_name)
            .key(key)
            .presigned(presigning_config)
            .await?;

        Ok(presigned_request.uri().to_string())
    }

    /// 生成预签名上传链接
    pub async fn generate_upload_url(&self, key: &str) -> Result<String, Error> {
        let presigning_config = PresigningConfig::expires_in(Duration::from_secs(15 * 60))?;

        let presigned_request = self.client
            .put_object()
            .bucket(&self.bucket_name)
            .key(key)
            .presigned(presigning_config)
            .await?;

        Ok(presigned_request.uri().to_string())
    }

    /// 检查对象是否存在
    pub async fn object_exists(&self, key: &str) -> Result<bool, Error> {
        match self.client
            .head_object()
            .bucket(&self.bucket_name)
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
                    Err(err)
                }
            }
        }
    }

    /// 获取对象元数据
    pub async fn get_object_metadata(&self, key: &str) -> Result<ObjectMetadata, Error> {
        let response = self.client
            .head_object()
            .bucket(&self.bucket_name)
            .key(key)
            .send()
            .await?;

        Ok(ObjectMetadata {
            key: key.to_string(),
            size: response.content_length().unwrap_or(0),
            last_modified: response.last_modified().map(|dt| dt.to_string()),
            content_type: response.content_type().map(|s| s.to_string()),
            etag: response.e_tag().map(|s| s.to_string()),
        })
    }
}

/// 对象元数据结构
#[derive(Debug, Clone)]
pub struct ObjectMetadata {
    pub key: String,
    pub size: i64,
    pub last_modified: Option<String>,
    pub content_type: Option<String>,
    pub etag: Option<String>,
}

/// 示例使用方法
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn example_usage() -> Result<(), Error> {
        // 创建 S3 管理器
        let s3_manager = S3Manager::new("your-bucket-name".to_string()).await?;

        // 测试连接
        s3_manager.test_connection().await?;

        // 列出所有对象
        let objects = s3_manager.list_all_objects().await?;
        println!("Found {} objects", objects.len());

        // 生成下载链接
        if let Some(first_object) = objects.first() {
            let download_url = s3_manager.generate_download_url(first_object).await?;
            println!("Download URL: {}", download_url);
        }

        // 删除对象
        // s3_manager.delete_object("path/to/file.txt").await?;

        Ok(())
    }
}