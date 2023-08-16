/// Messages used in calls to track user's current view to provide appropriate IsTyping messages.
/// e.g. Only when a user is on the same peer as the user who is typing, the user will receive the IsTyping message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectRequest {
    #[prost(string, tag = "1")]
    pub app_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectPeerRequest {
    #[prost(string, tag = "1")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub peer_id: i64,
}
/// Messages used by the Updates service. This is the primary Response vehicle for real-time updates to
/// connected devices -- used by the `Connect` gRPC streaming response method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatesStream {
    #[prost(oneof = "updates_stream::Update", tags = "5, 6, 7")]
    pub update: ::core::option::Option<updates_stream::Update>,
}
/// Nested message and enum types in `UpdatesStream`.
pub mod updates_stream {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Update {
        /// for returning existing Comments
        #[prost(message, tag = "5")]
        Comment(super::CommentSent),
        /// for returning real-time updates
        #[prost(message, tag = "6")]
        Reaction(super::Reaction),
        /// for returning real-time is_typing messages
        #[prost(message, tag = "7")]
        UserIsTyping(super::UserIsTypingResponse),
    }
}
/// *******************************************
/// Topic level messages that pushed to clients
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommentSent {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(int64, tag = "2")]
    pub topic_id: i64,
    #[prost(int64, tag = "3")]
    pub user_id: i64,
    #[prost(string, tag = "4")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub replyto_comment_id: i64,
    #[prost(string, tag = "6")]
    pub comment: ::prost::alloc::string::String,
    #[prost(bool, tag = "7")]
    pub this_users_comment: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reaction {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(string, tag = "3")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub comment_id: i64,
    #[prost(string, tag = "5")]
    pub reaction: ::prost::alloc::string::String,
    #[prost(bool, tag = "6")]
    pub this_users_reaction: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserIsTypingResponse {
    #[prost(string, tag = "1")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub peer_id: i64,
    #[prost(int64, optional, tag = "3")]
    pub replying_to_id: ::core::option::Option<i64>,
}
/// Generated client implementations.
pub mod updates_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service reponsible for pushing all real-time chat-related updates to the client
    #[derive(Debug, Clone)]
    pub struct UpdatesServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl UpdatesServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> UpdatesServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> UpdatesServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            UpdatesServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// `Connect` is called when the user opens the app and is ready to receive updates
        /// This is a server-side stream RPC, meaning that the server will keep the connection open
        /// and send updates to the client as they happen
        pub async fn updates_connect(
            &mut self,
            request: impl tonic::IntoRequest<super::ConnectRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::UpdatesStream>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/updates_stream.UpdatesService/UpdatesConnect",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("updates_stream.UpdatesService", "UpdatesConnect"),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        /// `RegisterOnPeerList` is called when the user opens the chat screen containing the
        /// list of their peers (e.g. chats, chatgroups)
        pub async fn register_on_peer_list(
            &mut self,
            request: impl tonic::IntoRequest<super::ConnectRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/updates_stream.UpdatesService/RegisterOnPeerList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "updates_stream.UpdatesService",
                        "RegisterOnPeerList",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// `UnregisterFromPeerList` is called when the user opens the chat screen containing the
        /// list of their peers (e.g. chats, chatgroups)
        pub async fn unregister_from_peer_list(
            &mut self,
            request: impl tonic::IntoRequest<super::ConnectRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/updates_stream.UpdatesService/UnregisterFromPeerList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "updates_stream.UpdatesService",
                        "UnregisterFromPeerList",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// `ActiveOnPeer` is called when the user opens a specific chat screen (e.g. chat, chatgroup, topic, channel)
        pub async fn register_on_peer(
            &mut self,
            request: impl tonic::IntoRequest<super::ConnectPeerRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/updates_stream.UpdatesService/RegisterOnPeer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("updates_stream.UpdatesService", "RegisterOnPeer"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// `LeavePeer` is called when the user leaves a specific chat screen (e.g. chat, chatgroup, topic, channel)
        pub async fn unregister_from_peer(
            &mut self,
            request: impl tonic::IntoRequest<super::ConnectPeerRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/updates_stream.UpdatesService/UnregisterFromPeer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "updates_stream.UpdatesService",
                        "UnregisterFromPeer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod updates_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with UpdatesServiceServer.
    #[async_trait]
    pub trait UpdatesService: Send + Sync + 'static {
        /// Server streaming response type for the UpdatesConnect method.
        type UpdatesConnectStream: futures_core::Stream<
                Item = std::result::Result<super::UpdatesStream, tonic::Status>,
            >
            + Send
            + 'static;
        /// `Connect` is called when the user opens the app and is ready to receive updates
        /// This is a server-side stream RPC, meaning that the server will keep the connection open
        /// and send updates to the client as they happen
        async fn updates_connect(
            &self,
            request: tonic::Request<super::ConnectRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::UpdatesConnectStream>,
            tonic::Status,
        >;
        /// `RegisterOnPeerList` is called when the user opens the chat screen containing the
        /// list of their peers (e.g. chats, chatgroups)
        async fn register_on_peer_list(
            &self,
            request: tonic::Request<super::ConnectRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// `UnregisterFromPeerList` is called when the user opens the chat screen containing the
        /// list of their peers (e.g. chats, chatgroups)
        async fn unregister_from_peer_list(
            &self,
            request: tonic::Request<super::ConnectRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// `ActiveOnPeer` is called when the user opens a specific chat screen (e.g. chat, chatgroup, topic, channel)
        async fn register_on_peer(
            &self,
            request: tonic::Request<super::ConnectPeerRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /// `LeavePeer` is called when the user leaves a specific chat screen (e.g. chat, chatgroup, topic, channel)
        async fn unregister_from_peer(
            &self,
            request: tonic::Request<super::ConnectPeerRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
    }
    /// Service reponsible for pushing all real-time chat-related updates to the client
    #[derive(Debug)]
    pub struct UpdatesServiceServer<T: UpdatesService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: UpdatesService> UpdatesServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for UpdatesServiceServer<T>
    where
        T: UpdatesService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/updates_stream.UpdatesService/UpdatesConnect" => {
                    #[allow(non_camel_case_types)]
                    struct UpdatesConnectSvc<T: UpdatesService>(pub Arc<T>);
                    impl<
                        T: UpdatesService,
                    > tonic::server::ServerStreamingService<super::ConnectRequest>
                    for UpdatesConnectSvc<T> {
                        type Response = super::UpdatesStream;
                        type ResponseStream = T::UpdatesConnectStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConnectRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).updates_connect(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdatesConnectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/updates_stream.UpdatesService/RegisterOnPeerList" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterOnPeerListSvc<T: UpdatesService>(pub Arc<T>);
                    impl<
                        T: UpdatesService,
                    > tonic::server::UnaryService<super::ConnectRequest>
                    for RegisterOnPeerListSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConnectRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).register_on_peer_list(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RegisterOnPeerListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/updates_stream.UpdatesService/UnregisterFromPeerList" => {
                    #[allow(non_camel_case_types)]
                    struct UnregisterFromPeerListSvc<T: UpdatesService>(pub Arc<T>);
                    impl<
                        T: UpdatesService,
                    > tonic::server::UnaryService<super::ConnectRequest>
                    for UnregisterFromPeerListSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConnectRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).unregister_from_peer_list(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnregisterFromPeerListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/updates_stream.UpdatesService/RegisterOnPeer" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterOnPeerSvc<T: UpdatesService>(pub Arc<T>);
                    impl<
                        T: UpdatesService,
                    > tonic::server::UnaryService<super::ConnectPeerRequest>
                    for RegisterOnPeerSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConnectPeerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).register_on_peer(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RegisterOnPeerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/updates_stream.UpdatesService/UnregisterFromPeer" => {
                    #[allow(non_camel_case_types)]
                    struct UnregisterFromPeerSvc<T: UpdatesService>(pub Arc<T>);
                    impl<
                        T: UpdatesService,
                    > tonic::server::UnaryService<super::ConnectPeerRequest>
                    for UnregisterFromPeerSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConnectPeerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).unregister_from_peer(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnregisterFromPeerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: UpdatesService> Clone for UpdatesServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: UpdatesService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: UpdatesService> tonic::server::NamedService for UpdatesServiceServer<T> {
        const NAME: &'static str = "updates_stream.UpdatesService";
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetadataMap {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub value: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ************ RegisterUpdatesConnection ************ //
/// Backend NATS messaging for `Connect`: When a client makes a
/// `ConnectForUpdates` method call to the chat-server, the chat-server in turn
/// makes an internal NATS request register the connection in the shared REDIS Presence cache.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsUpdatesConnectRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<UpdatesConnection>,
}
/// ************ UnregisterUpdatesConnection ************ //
/// Backend NATS messaging to close an Updates connection when it is disconnected
/// There is no cooresponding client method call for this message as it is triggered by the connection closing.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsUpdatesCloseRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<UpdatesConnection>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatesConnection {
    #[prost(message, optional, tag = "1")]
    pub connection_key: ::core::option::Option<UpdatesConnectionKey>,
    /// host:port of the chat server that owns the connection
    #[prost(string, tag = "3")]
    pub chat_server: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatesConnectionKey {
    /// user_id of the user who is connected for updates
    #[prost(int64, tag = "1")]
    pub user_id: i64,
    /// unique client app name that recieves the updates
    #[prost(string, tag = "2")]
    pub app_id: ::prost::alloc::string::String,
}
/// ************ RegisterOnPeerList ************ //
/// Backend NATS messaging for `RegisterOnPeerList`: When a client makes a
/// `RegisterOnPeerList` method call to the chat-server, the chat-server in turn
/// makes an internal NATS request to register the peer-list as active for this
/// connection with the shared REDIS Presence cache.
/// This is used to determine who should receive is typing notifications.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsRegisterOnPeerListRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<UpdatesConnectionKey>,
}
/// ************ UnregisterFromPeerList ************ //
/// Backend NATS messaging for `UnregisterFromPeerList`: When a client makes a
/// `UnregisterFromPeerList` method call to the chat-server, the chat-server in turn
/// makes an internal NATS request to register the peer-list as active for this
/// connection with the shared REDIS Presence cache.
/// This is used to determine who should receive is typing notifications.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsUnregisterFromPeerListRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<UpdatesConnectionKey>,
}
/// ************ RegisterOnPeer ************ //
/// Backend NATS messaging for `RegisterOnPeer`: When a client makes a
/// `RegisterOnPeer` method call to the chat-server, the chat-server in turn
/// makes an internal NATS request to register the current active Peer with the shared REDIS Presence cache.
/// This is used to determine who should receive is typing notifications.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsRegisterOnPeerRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<ConnectionPeer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionPeer {
    /// updates connection key associated with this peer registration
    #[prost(message, optional, tag = "1")]
    pub connection_key: ::core::option::Option<UpdatesConnectionKey>,
    /// peer_id of the peer that is active
    #[prost(int64, tag = "2")]
    pub peer_id: i64,
}
/// ************ UnregisterFromPeer ************ //
/// Backend NATS messaging for `LeaveUnregisterFromPeerPeer`: When a client makes a
/// `UnregisterFromPeer` method call to the chat-server, the chat-server in turn
/// makes an internal NATS request to unregister the current active Peer with the shared REDIS Presence cache.
/// This is used to stop sending is typing notifications to this connection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsUnregisterFromPeerRequest {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<MetadataMap>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<ConnectionPeer>,
}
/// External Error Response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    #[prost(int32, tag = "1")]
    pub code: i32,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub status: i32,
}
/// Internal/NATS Error Response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorReply {
    #[prost(int32, tag = "1")]
    pub code: i32,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub status: i32,
    #[prost(message, repeated, tag = "4")]
    pub details: ::prost::alloc::vec::Vec<ErrorDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorDetails {
    #[prost(string, tag = "1")]
    pub reason: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub domain: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub metadata: ::prost::alloc::vec::Vec<MetaData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetaData {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NatsEmptyResponse {
    #[prost(oneof = "nats_empty_response::Msg", tags = "1, 2")]
    pub msg: ::core::option::Option<nats_empty_response::Msg>,
}
/// Nested message and enum types in `NatsEmptyResponse`.
pub mod nats_empty_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Msg {
        #[prost(bool, tag = "1")]
        Success(bool),
        #[prost(message, tag = "2")]
        Error(super::ErrorReply),
    }
}
