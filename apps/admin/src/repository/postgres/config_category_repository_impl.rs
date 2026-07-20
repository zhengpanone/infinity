use crate::domain::dto::config_category::{ConfigCategoryQueryDTO, ConfigCategorySortField};
use crate::domain::types::ids::ConfigCategoryId;
use crate::repository::config_category_repository::{
    CheckConfigCategoryExists, ConfigCategoryExistsResult, UpdateConfigCategory,
};
use crate::repository::config_category_repository::{ConfigCategoryRepository, NewConfigCategory};
use infinity_error::{ErrorKind, InfinityError, Result, ResultExt};
use infinity_web::{PaginatedData, PaginationParams, SortRule};
use sqlx::{PgPool, Postgres, QueryBuilder};
use tracing::debug;
use crate::models::config_category::ConfigCategory;

/// `sys_role` 全字段列，展开为字符串字面量，供查询/返回复用。
macro_rules! config_category_columns {
    () => {
        r#"id, category_code, category_name, icon, color,
           order_num, remark, category_desc, is_builtin,
           created_id, created_at, created_by,
           updated_id, updated_at, updated_by,
           is_deleted, deleted_at"#
    };
}

pub struct ConfigCategoryRepositoryImpl {
    pool: PgPool,
}

impl ConfigCategoryRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ConfigCategoryRepository for ConfigCategoryRepositoryImpl {
    async fn create(&self, config_category: NewConfigCategory) -> Result<ConfigCategory> {
        let saved = sqlx::query_as::<_, ConfigCategory>(concat!(
            r#"INSERT INTO sys_config_category (
                id, category_code, category_name, icon, color, order_num, remark, category_desc, is_builtin
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9
            ) RETURNING "#,
            config_category_columns!()
        ))
        .bind(ConfigCategoryId::generate().as_uuid())
        .bind(config_category.category_code)
        .bind(config_category.category_name)
        .bind(config_category.icon)
        .bind(config_category.color)
        .bind(config_category.order_num)
        .bind(config_category.remark)
        .bind(config_category.category_desc)
        .bind(config_category.is_builtin)
        .fetch_one(&self.pool)
        .await
        .context(ErrorKind::Database, "failed to create config category")?;
        Ok(saved)
    }

    async fn update_by_id(&self, user: UpdateConfigCategory) -> Result<Option<ConfigCategory>> {
        let updated = sqlx::query_as::<_, ConfigCategory>(concat!(
            r#"UPDATE sys_config_category SET
                category_code = COALESCE($2, category_code),
                category_name = COALESCE($3, category_name),
                icon = COALESCE($4, icon),
                color = COALESCE($5, color),
                order_num = COALESCE($6, order_num),
                remark = COALESCE($7, remark),
                category_desc = COALESCE($8, category_desc),
                is_builtin = COALESCE($9, is_builtin),
                updated_id = COALESCE($10, updated_id),
                updated_at = NOW()
            WHERE id = $1 AND is_deleted = FALSE
            RETURNING "#,
            config_category_columns!()
        ))
        .bind(user.id)
        .bind(user.category_code)
        .bind(user.category_name)
        .bind(user.icon)
        .bind(user.color)
        .bind(user.order_num)
        .bind(user.remark)
        .bind(user.category_desc)
        .bind(user.is_builtin)
        .bind("1")
        .fetch_optional(&self.pool)
        .await
        .context(ErrorKind::Database, "failed to update config category")?;
        Ok(updated)
    }

    async fn soft_delete(&self, ids: &[ConfigCategoryId]) -> Result<u64> {
        if ids.is_empty() {
            return Ok(0);
        }
        let ids = ids.iter().map(|id| id.as_uuid()).collect::<Vec<_>>();
        let result = sqlx::query(
            r#"UPDATE sys_config_category
                SET is_deleted = TRUE,
                    status = 'deleted',
                    deleted_at = NOW()
                WHERE id = ANY($1) AND is_deleted = FALSE"#,
        )
        .bind(ids)
        .execute(&self.pool)
        .await
        .context(ErrorKind::Database, "failed to soft delete config category")?;
        Ok(result.rows_affected())
    }

    async fn find_by_id(&self, id: &ConfigCategoryId) -> Result<Option<ConfigCategory>> {
        let result = sqlx::query_as::<_, ConfigCategory>(concat!(
            "SELECT ",
            config_category_columns!(),
            " FROM sys_config_category WHERE id = $1 AND is_deleted = FALSE"
        ))
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .context(ErrorKind::Database, "failed to find config category by id")?;
        Ok(result)
    }

    async fn page_list(
        &self,
        query: PaginationParams<ConfigCategoryQueryDTO, ConfigCategorySortField>,
    ) -> Result<PaginatedData<Vec<ConfigCategory>>> {
        let limit = query.limit().map_err(InfinityError::validation)?;

        let offset = query.offset().map_err(InfinityError::validation)?;

        let mut count_builder = QueryBuilder::<Postgres>::new(
            r#"SELECT COUNT(1)::BIGINT
            FROM sys_config_category
            WHERE is_deleted = FALSE"#,
        );
        push_config_category_filters(&mut count_builder, &query.filters);
        let total: i64 = count_builder
            .build_query_scalar::<i64>()
            .fetch_one(&self.pool)
            .await
            .context(ErrorKind::Database, "failed to count config category")?;
        let mut data_builder = QueryBuilder::<Postgres>::new("SELECT ");
        data_builder.push(config_category_columns!()).push(
            r#" FROM sys_config_category
                WHERE is_deleted = FALSE
        "#,
        );
        push_config_category_filters(&mut data_builder, &query.filters);
        push_config_category_sorts(&mut data_builder, &query.sorts);

        data_builder
            .push(" LIMIT ")
            .push_bind(limit)
            .push(" OFFSET ")
            .push_bind(offset);
        let config_categories = data_builder
            .build_query_as::<ConfigCategory>()
            .fetch_all(&self.pool)
            .await
            .context(ErrorKind::Database, "failed to find config category by id")?;

        let total =
            u64::try_from(total).context(ErrorKind::Database, "failed to count config category")?;

        PaginatedData::try_new(config_categories, query.page_num, query.page_size, total)
            .map_err(InfinityError::validation)
    }

    async fn check_exists(
        &self,
        query: &CheckConfigCategoryExists,
    ) -> Result<ConfigCategoryExistsResult> {
        let category_code = query.category_code.as_ref().map(|value| value.as_str());

        let exclude_id = query
            .exclude_category_id
            .as_ref()
            .map(ConfigCategoryId::as_uuid);
        let result: (Option<bool>,) = sqlx::query_as(
            r#"SELECT
                CASE WHEN $1::VARCHAR IS NULL THEN NULL
                ELSE EXISTS (
                    SELECT 1
                    FROM sys_config_category
                    WHERE category_code = $1
                    AND is_deleted = FALSE
                    AND ($2::UUID IS NULL OR id <> $2)
                ) END AS category_code_exists
               "#,
        )
        .bind(category_code)
        .bind(exclude_id)
        .fetch_one(&self.pool)
        .await
        .context(ErrorKind::Database, "failed to check user exists")?;
        Ok(ConfigCategoryExistsResult {
            category_code_exists: result.0,
        })
    }
}

fn push_config_category_filters(
    builder: &mut QueryBuilder<Postgres>,
    filters: &ConfigCategoryQueryDTO,
) {
    if let Some(category_code) = filters
        .category_code
        .as_deref()
        .filter(|value| !value.is_empty())
    {
        builder
            .push(" AND category_code = ")
            .push_bind(category_code);
    }
    if let Some(category_name) = filters
        .category_name
        .as_deref()
        .filter(|value| !value.is_empty())
    {
        builder
            .push(" AND category_name LIKE ")
            .push_bind(format!("%{category_name}%"));
    }
}

fn push_config_category_sorts(
    builder: &mut QueryBuilder<Postgres>,
    sorts: &[SortRule<ConfigCategorySortField>],
) {
    builder.push(" ORDER BY ");
    if sorts.is_empty() {
        builder.push("created_at DESC, id ASC");
        return;
    }

    for (index, sort) in sorts.iter().enumerate() {
        if index > 0 {
            builder.push(", ");
        }
        builder.push(sort.field.as_sql());
        builder.push(" ");
        builder.push(sort.order.as_sql());
    }
    builder.push(" , id ASC");
}
