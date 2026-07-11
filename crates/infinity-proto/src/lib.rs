pub mod common {
    tonic::include_proto!("com.zp.infinity.common");
}

pub mod hello {
    tonic::include_proto!("com.zp.infinity.admin");
}

pub mod user {
    tonic::include_proto!("com.zp.infinity.admin.user");
}

// ✅ 给 Reflection 使用
pub const FILE_DESCRIPTOR_SET: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/grpc_descriptor.bin"));
