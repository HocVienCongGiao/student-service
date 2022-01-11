//! Main library entry point for openapi_client implementation.

#![allow(unused_imports)]

use async_trait::async_trait;
use futures::{future, Stream, StreamExt, TryFutureExt, TryStreamExt};
use hyper::server::conn::Http;
use hyper::service::Service;
use log::info;
#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::SslAcceptorBuilder;
use std::future::Future;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swagger::{Has, XSpanIdString};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::EmptyContext;
use tokio::net::TcpListener;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use openapi_client::models;

/// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
pub async fn create(addr: &str, https: bool) {
    let addr = addr.parse().expect("Failed to parse bind address");

    let server = Server::new();

    let service = MakeService::new(server);

    let service = MakeAllowAllAuthenticator::new(service, "cosmo");

    let mut service =
        openapi_client::server::context::MakeAddContext::<_, EmptyContext>::new(
            service
        );

    if https {
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
        {
            unimplemented!("SSL is not implemented for the examples on MacOS, Windows or iOS");
        }

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
        {
            let mut ssl = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).expect("Failed to create SSL Acceptor");

            // Server authentication
            ssl.set_private_key_file("examples/server-key.pem", SslFiletype::PEM).expect("Failed to set private key");
            ssl.set_certificate_chain_file("examples/server-chain.pem").expect("Failed to set cerificate chain");
            ssl.check_private_key().expect("Failed to check private key");

            let tls_acceptor = Arc::new(ssl.build());
            let mut tcp_listener = TcpListener::bind(&addr).await.unwrap();
            let mut incoming = tcp_listener.incoming();

            while let (Some(tcp), rest) = incoming.into_future().await {
                if let Ok(tcp) = tcp {
                    let addr = tcp.peer_addr().expect("Unable to get remote address");
                    let service = service.call(addr);
                    let tls_acceptor = Arc::clone(&tls_acceptor);

                    tokio::spawn(async move {
                        let tls = tokio_openssl::accept(&*tls_acceptor, tcp).await.map_err(|_| ())?;

                        let service = service.await.map_err(|_| ())?;

                        Http::new().serve_connection(tls, service).await.map_err(|_| ())
                    });
                }

                incoming = rest;
            }
        }
    } else {
        // Using HTTP
        hyper::server::Server::bind(&addr).serve(service).await.unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}


use openapi_client::{
    Api,
    AddStudentResponse,
    DeleteStudentResponse,
    UpdateStudentResponse,
    GetStudentByIdResponse,
    GetStudentsResponse,
};
use openapi_client::server::MakeService;
use std::error::Error;
use swagger::ApiError;

#[async_trait]
impl<C> Api<C> for Server<C> where C: Has<XSpanIdString> + Send + Sync
{
    /// Add new student
    async fn add_student(
        &self,
        student_upsert: models::StudentUpsert,
        context: &C) -> Result<AddStudentResponse, ApiError>
    {
        let context = context.clone();
        info!("add_student({:?}) - X-Span-ID: {:?}", student_upsert, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Deletes a student
    async fn delete_student(
        &self,
        id: uuid::Uuid,
        context: &C) -> Result<DeleteStudentResponse, ApiError>
    {
        let context = context.clone();
        info!("delete_student({:?}) - X-Span-ID: {:?}", id, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Update an existing student
    async fn update_student(
        &self,
        id: uuid::Uuid,
        student_upsert: models::StudentUpsert,
        context: &C) -> Result<UpdateStudentResponse, ApiError>
    {
        let context = context.clone();
        info!("update_student({:?}, {:?}) - X-Span-ID: {:?}", id, student_upsert, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Find student by ID
    async fn get_student_by_id(
        &self,
        id: uuid::Uuid,
        context: &C) -> Result<GetStudentByIdResponse, ApiError>
    {
        let context = context.clone();
        info!("get_student_by_id({:?}) - X-Span-ID: {:?}", id, context.get().0.clone());
        Err("Generic failuare".into())
    }

    /// Get all students
    async fn get_students(
        &self,
        name: Option<String>,
        email: Option<String>,
        phone: Option<String>,
        undergraduate_school: Option<String>,
        date_of_birth: Option<chrono::DateTime::<chrono::Utc>>,
        place_of_birth: Option<String>,
        polity_name: Option<String>,
        specialism: Option<String>,
        sorts: Option<&Vec<models::StudentSortCriteria>>,
        offset: Option<i32>,
        count: Option<i32>,
        context: &C) -> Result<GetStudentsResponse, ApiError>
    {
        let context = context.clone();
        info!("get_students({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}) - X-Span-ID: {:?}", name, email, phone, undergraduate_school, date_of_birth, place_of_birth, polity_name, specialism, sorts, offset, count, context.get().0.clone());
        Err("Generic failuare".into())
    }

}
