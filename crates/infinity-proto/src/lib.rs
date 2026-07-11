// 模块层级与 proto 的 package 路径一致（infinity.common / infinity.admin.*），
// 这样 prost 生成的跨 package 引用（super::...）才能正确解析。
pub mod infinity {
    pub mod common {
        tonic::include_proto!("infinity.common");
    }
    pub mod admin {
        pub mod hello {
            tonic::include_proto!("infinity.admin.hello");
        }
        pub mod user {
            tonic::include_proto!("infinity.admin.user");
        }
    }
}

// 顶层再导出，保持既有调用路径：infinity_proto::{common, hello, user}
pub use infinity::admin::hello;
pub use infinity::admin::user;
pub use infinity::common;

// ✅ 给 Reflection 使用
pub const FILE_DESCRIPTOR_SET: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/grpc_descriptor.bin"));
