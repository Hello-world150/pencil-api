use crate::{AppError, User};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation_error() {
        // 测试空用户名应该返回错误
        let result = User::new("".to_string());
        assert!(result.is_err());
        
        if let Err(AppError::User(msg)) = result {
            assert_eq!(msg, "用户名不能为空");
        } else {
            panic!("Expected AppError::User");
        }
    }

    #[test] 
    fn test_user_creation_success() {
        // 测试有效用户名应该成功
        let result = User::new("test_user".to_string());
        assert!(result.is_ok());
        
        let user = result.unwrap();
        assert_eq!(user.username, "test_user");
        assert!(user.items.is_empty());
        assert!(user.collections.is_empty());
    }

    #[test]
    fn test_error_response_format() {
        // 测试错误响应格式
        let error = AppError::User("测试用户错误".to_string());
        let response = error.to_response();
        
        assert_eq!(response.error, "用户错误: 测试用户错误");
        assert_eq!(response.code, "USER_ERROR");
    }

    #[test]
    fn test_different_error_types() {
        // 测试不同类型的错误
        let errors = vec![
            AppError::User("用户错误".to_string()),
            AppError::Collection("文集错误".to_string()),
            AppError::NotFound("资源不存在".to_string()),
            AppError::AlreadyExists("资源已存在".to_string()),
            AppError::Validation("验证错误".to_string()),
        ];

        for error in errors {
            let response = error.to_response();
            assert!(!response.error.is_empty());
            assert!(!response.code.is_empty());
        }
    }
}
