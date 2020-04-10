// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait Bot {
    fn exec(&self, o: ::grpc::RequestOptions, p: super::bot::ExecRequest) -> ::grpc::SingleResponse<super::bot::ExecResponse>;

    fn tail(&self, o: ::grpc::RequestOptions, p: super::bot::TailRequest) -> ::grpc::StreamingResponse<super::bot::TailResponse>;

    fn spawn(&self, o: ::grpc::RequestOptions, p: super::bot::SpawnRequest) -> ::grpc::SingleResponse<super::bot::SpawnResponse>;

    fn copy_file(&self, o: ::grpc::RequestOptions, p: super::bot::CopyFileRequest) -> ::grpc::SingleResponse<super::bot::CopyFileResponse>;

    fn copy_dir(&self, o: ::grpc::RequestOptions, p: super::bot::CopyDirRequest) -> ::grpc::SingleResponse<super::bot::CopyDirResponse>;

    fn assign_dir(&self, o: ::grpc::RequestOptions, p: super::bot::AssignDirRequest) -> ::grpc::SingleResponse<super::bot::AssignDirResponse>;

    fn read_file(&self, o: ::grpc::RequestOptions, p: super::bot::ReadFileRequest) -> ::grpc::SingleResponse<super::bot::ReadFileResponse>;

    fn write_file(&self, o: ::grpc::RequestOptions, p: super::bot::WriteFileRequest) -> ::grpc::SingleResponse<super::bot::WriteFileResponse>;
}

// client

pub struct BotClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_Exec: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::bot::ExecRequest, super::bot::ExecResponse>>,
    method_Tail: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::bot::TailRequest, super::bot::TailResponse>>,
    method_Spawn: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::bot::SpawnRequest, super::bot::SpawnResponse>>,
    method_CopyFile: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::bot::CopyFileRequest, super::bot::CopyFileResponse>>,
    method_CopyDir: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::bot::CopyDirRequest, super::bot::CopyDirResponse>>,
    method_AssignDir: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::bot::AssignDirRequest, super::bot::AssignDirResponse>>,
    method_ReadFile: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::bot::ReadFileRequest, super::bot::ReadFileResponse>>,
    method_WriteFile: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::bot::WriteFileRequest, super::bot::WriteFileResponse>>,
}

impl ::grpc::ClientStub for BotClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        BotClient {
            grpc_client: grpc_client,
            method_Exec: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/Bot/Exec".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Tail: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/Bot/Tail".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Spawn: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/Bot/Spawn".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_CopyFile: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/Bot/CopyFile".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_CopyDir: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/Bot/CopyDir".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_AssignDir: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/Bot/AssignDir".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ReadFile: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/Bot/ReadFile".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_WriteFile: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/Bot/WriteFile".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl Bot for BotClient {
    fn exec(&self, o: ::grpc::RequestOptions, p: super::bot::ExecRequest) -> ::grpc::SingleResponse<super::bot::ExecResponse> {
        self.grpc_client.call_unary(o, p, self.method_Exec.clone())
    }

    fn tail(&self, o: ::grpc::RequestOptions, p: super::bot::TailRequest) -> ::grpc::StreamingResponse<super::bot::TailResponse> {
        self.grpc_client.call_server_streaming(o, p, self.method_Tail.clone())
    }

    fn spawn(&self, o: ::grpc::RequestOptions, p: super::bot::SpawnRequest) -> ::grpc::SingleResponse<super::bot::SpawnResponse> {
        self.grpc_client.call_unary(o, p, self.method_Spawn.clone())
    }

    fn copy_file(&self, o: ::grpc::RequestOptions, p: super::bot::CopyFileRequest) -> ::grpc::SingleResponse<super::bot::CopyFileResponse> {
        self.grpc_client.call_unary(o, p, self.method_CopyFile.clone())
    }

    fn copy_dir(&self, o: ::grpc::RequestOptions, p: super::bot::CopyDirRequest) -> ::grpc::SingleResponse<super::bot::CopyDirResponse> {
        self.grpc_client.call_unary(o, p, self.method_CopyDir.clone())
    }

    fn assign_dir(&self, o: ::grpc::RequestOptions, p: super::bot::AssignDirRequest) -> ::grpc::SingleResponse<super::bot::AssignDirResponse> {
        self.grpc_client.call_unary(o, p, self.method_AssignDir.clone())
    }

    fn read_file(&self, o: ::grpc::RequestOptions, p: super::bot::ReadFileRequest) -> ::grpc::SingleResponse<super::bot::ReadFileResponse> {
        self.grpc_client.call_unary(o, p, self.method_ReadFile.clone())
    }

    fn write_file(&self, o: ::grpc::RequestOptions, p: super::bot::WriteFileRequest) -> ::grpc::SingleResponse<super::bot::WriteFileResponse> {
        self.grpc_client.call_unary(o, p, self.method_WriteFile.clone())
    }
}

// server

pub struct BotServer;


impl BotServer {
    pub fn new_service_def<H : Bot + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/Bot",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/Bot/Exec".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.exec(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/Bot/Tail".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.tail(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/Bot/Spawn".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.spawn(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/Bot/CopyFile".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.copy_file(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/Bot/CopyDir".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.copy_dir(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/Bot/AssignDir".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.assign_dir(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/Bot/ReadFile".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.read_file(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/Bot/WriteFile".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.write_file(o, p))
                    },
                ),
            ],
        )
    }
}
