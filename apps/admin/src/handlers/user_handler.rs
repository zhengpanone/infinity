const TAG_NAME: &str = "User API";
const ADMIN_TAG_NAME: &str = "Admin User API";

// 创建用户
// #[utoipa::path(
//     post,
//     path = "",
//     tag = ADMIN_TAG_NAME,
//     request_body = CreateUserRequest,
//     responses(
//         (status = 200, description = "创建成功", body = UserResponse),
//         (status = 400, description = "请求参数错误"),
//         (status = 401, description = "未授权"),
//         (status = 403, description = "权限不足"),
//         (status = 409, description = "用户已存在")
//     ),
//     security(
//         ("jwt" = [])
//     )
// )]
// // #[instrument(name="http_create_user", skip_all, fields(user_id=%auth_user.user_id))]
// pub async fn create_user(
//     State(state): State<Arc<AppState>>,
//     // auth_user: AuthUser,
//     Json(payload): Json<CreateUserRequest>,
// ) -> AppResult<ApiResponse<UserResponse>> {
//     // 检查权限
//     // require_role(&auth_user, "admin")?;
//     debug!("Create user: {:?}", payload);

//     Ok(ApiResponse::created(resp))
// }
