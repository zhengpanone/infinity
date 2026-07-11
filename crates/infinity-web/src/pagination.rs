use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use utoipa::ToSchema;

const DEFAULT_PAGE: u64 = 1;
const DEFAULT_PAGE_SIZE: u64 = 20;
const MAX_PAGE_SIZE: u64 = 100;

const fn default_page() -> u64 {
    DEFAULT_PAGE
}

const fn default_page_size() -> u64 {
    DEFAULT_PAGE_SIZE
}

/// 分页链接
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginationLinks {
    /// 首页链接
    pub first: Option<String>,

    /// 上一页链接
    pub prev: Option<String>,

    /// 当前页链接
    pub current: String,

    /// 下一页链接
    pub next: Option<String>,

    /// 末页链接
    pub last: Option<String>,
}

/// 分页数据
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginatedData<T> {
    /// 数据项
    pub items: T,

    /// 当前页码
    pub page: u64,

    /// 每页数量
    pub page_size: u64,

    /// 总记录数
    pub total: u64,

    /// 总页数
    pub total_pages: u64,

    /// 是否有上一页
    pub has_previous: bool,

    /// 是否有下一页
    pub has_next: bool,

    /// 上一页页码
    pub previous_page: Option<u64>,

    /// 下一页页码
    pub next_page: Option<u64>,
}

impl<T> PaginatedData<T> {
    /// 创建分页数据
    pub fn try_new(items: T, page: u64, page_size: u64, total: u64) -> Result<Self, String> {
        if page == 0 {
            return Err("页码必须大于 0".to_string());
        }

        if page_size == 0 {
            return Err("每页数量必须大于 0".to_string());
        }
        let total_pages = total.div_ceil(page_size);
        let has_previous = page > 1;
        let has_next = page < total_pages;
        let previous_page = has_previous.then_some(page - 1);
        let next_page = has_next.then_some(page + 1);
        Ok(Self {
            items,
            page,
            page_size,
            total,
            total_pages,
            has_previous,
            has_next,
            previous_page,
            next_page,
        })
    }

    pub fn map<U>(self, mapper: impl FnOnce(T) -> U) -> PaginatedData<U> {
        PaginatedData {
            items: mapper(self.items),
            page: self.page,
            page_size: self.page_size,
            total: self.total,
            total_pages: self.total_pages,
            has_previous: self.has_previous,
            has_next: self.has_next,
            previous_page: self.previous_page,
            next_page: self.next_page,
        }
    }

    /// 获取当前页数据
    pub fn items(&self) -> &T {
        &self.items
    }

    /// 获取分页信息
    pub fn pagination_info(&self) -> PaginationInfo {
        PaginationInfo {
            page: self.page,
            page_size: self.page_size,
            total: self.total,
            total_pages: self.total_pages,
            has_previous: self.has_previous,
            has_next: self.has_next,
            previous_page: self.previous_page,
            next_page: self.next_page,
            links: None,
        }
    }
}

/// 分页信息
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginationInfo {
    /// 当前页码
    pub page: u64,

    /// 每页数量
    pub page_size: u64,

    /// 总记录数
    pub total: u64,

    /// 总页数
    pub total_pages: u64,

    /// 是否有上一页
    pub has_previous: bool,

    /// 是否有下一页
    pub has_next: bool,

    /// 上一页页码
    pub previous_page: Option<u64>,

    /// 下一页页码
    pub next_page: Option<u64>,

    /// 分页链接
    pub links: Option<PaginationLinks>,
}

/// 分页参数
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginationParams<F, S> {
    /// 页码，从1开始
    #[schema(default = default_page, minimum = 1)]
    #[serde(default = "default_page")]
    pub page_num: u64,

    /// 每页数量
    #[schema(default = default_page_size, minimum = 10)]
    #[serde(default = "default_page_size")]
    pub page_size: u64,

    /// 多字段排序, 数组顺序就是SQL排序优先级
    #[serde(default)]
    pub sorts: Vec<SortRule<S>>,

    /// 业务筛选条件
    ///
    /// `F::default()` 表示没有筛选条件
    #[serde(flatten)]
    pub filters: F,
}

impl<F, S> Default for PaginationParams<F, S>
where
    F: Default,
{
    fn default() -> Self {
        Self {
            page_num: default_page(),
            page_size: default_page_size(),
            sorts: Vec::new(),
            filters: F::default(),
        }
    }
}

impl<F, S> PaginationParams<F, S> {
    /// 创建分页参数
    pub fn new(page_num: u64, page_size: u64, filters: F) -> Self {
        Self {
            page_num,
            page_size,
            sorts: Vec::new(),
            filters,
        }
    }

    pub fn with_sort(mut self, field: S, order: SortOrder) -> Self {
        self.sorts.push(SortRule::new(field, order));
        self
    }
    pub fn with_sorts(mut self, sorts: impl IntoIterator<Item = SortRule<S>>) -> Self {
        self.sorts = sorts.into_iter().collect();
        self
    }

    /// 获取偏移量
    ///
    /// 假定调用方已先行 [`Self::validate`]；page_num 为 0 时仍会返回 `Err`，不会 panic。
    pub fn offset(&self) -> Result<i64, String> {
        let offset = self
            .page_num
            .checked_sub(1)
            .and_then(|page| page.checked_mul(self.page_size))
            .ok_or_else(|| "分页偏移量计算溢出".to_string())?;
        i64::try_from(offset).map_err(|_| "分页偏移量超过数据库支持的范围".to_string())
    }

    /// 获取限制
    ///
    /// 假定调用方已先行 [`Self::validate`]。
    pub fn limit(&self) -> Result<i64, String> {
        i64::try_from(self.page_size).map_err(|_| "每页数量超过数据库支持范围".to_string())
    }

    /// 验证分页参数
    pub fn validate(&self) -> Result<(), String> {
        if self.page_num < 1 {
            return Err("页码必须大于0".to_string());
        }

        if !(1..=MAX_PAGE_SIZE).contains(&self.page_size) {
            return Err(format!("每页数量必须在1-{MAX_PAGE_SIZE}之间"));
        }

        Ok(())
    }
}

/// 排序方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum SortOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

impl Default for SortOrder {
    fn default() -> Self {
        Self::Asc
    }
}

impl std::fmt::Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortOrder::Asc => write!(f, "asc"),
            SortOrder::Desc => write!(f, "desc"),
        }
    }
}

impl SortOrder {
    pub const fn as_sql(self) -> &'static str {
        match self {
            Self::Asc => "ASC",
            Self::Desc => "DESC",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct SortRule<S> {
    pub field: S,

    #[serde(default)]
    pub order: SortOrder,
}

impl<S> SortRule<S> {
    pub const fn new(field: S, order: SortOrder) -> Self {
        Self { field, order }
    }

    pub const fn asc(field: S) -> Self {
        Self::new(field, SortOrder::Asc)
    }
    pub const fn desc(field: S) -> Self {
        Self::new(field, SortOrder::Desc)
    }
}
