use crate::{AppError, User};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation_error_empty_username() {
        // 测试空用户名应该返回错误
        let result = User::new("".to_string(), "test_password123".to_string());
        assert!(result.is_err());

        if let Err(AppError::User(msg)) = result {
            assert_eq!(msg, "用户名不能为空");
        } else {
            panic!("Expected AppError::User for empty username");
        }
    }

    #[test]
    fn test_user_creation_error_empty_password() {
        // 测试空密码应该返回错误
        let result = User::new("test_user".to_string(), "".to_string());
        assert!(result.is_err());

        if let Err(AppError::User(msg)) = result {
            assert_eq!(msg, "密码不能为空");
        } else {
            panic!("Expected AppError::User for empty password");
        }
    }

    #[test]
    fn test_user_creation_error_whitespace_password() {
        // 测试仅包含空白字符的密码应该返回错误
        let result = User::new("test_user".to_string(), "   ".to_string());
        assert!(result.is_err());

        if let Err(AppError::User(msg)) = result {
            assert_eq!(msg, "密码不能为空");
        } else {
            panic!("Expected AppError::User for whitespace password");
        }
    }

    #[test]
    fn test_user_creation_success() {
        // 测试有效用户名和密码应该成功
        let password = "test_password123";
        let result = User::new("test_user".to_string(), password.to_string());
        assert!(result.is_ok());

        let user = result.unwrap();
        assert_eq!(user.username, "test_user");
        assert_eq!(
            pw_hash::bcrypt::verify("test_password123", user.hashed_password.as_ref().unwrap()),
            true
        );
        assert!(user.hitokotos.is_empty());
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
