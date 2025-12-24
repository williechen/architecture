use std::{
    pin::Pin,
    task::{Context, Poll},
};

use axum::{
    body::Body,
    http::{HeaderName, HeaderValue, Request, Response},
};
use tower::Layer;
use tower::Service;

#[derive(Clone)]
pub struct ContentSecurityPolicyLayer {
    base_uri: Option<String>,
    connect_src: Option<String>,
    default_src: Option<String>,
    font_src: Option<String>,
    frame_src: Option<String>,
    img_src: Option<String>,
    media_src: Option<String>,
    object_src: Option<String>,
    script_src: Option<String>,
    style_src: Option<String>,
    worker_src: Option<String>,
}

impl ContentSecurityPolicyLayer {
    pub fn new() -> Self {
        ContentSecurityPolicyLayer {
            base_uri: None,
            connect_src: None,
            default_src: None,
            font_src: None,
            frame_src: None,
            img_src: None,
            media_src: None,
            object_src: None,
            script_src: None,
            style_src: None,
            worker_src: None,
        }
    }
    // Additional builder methods can be added here to set each directive

    pub fn set_base_uri<T>(mut self, uri: T) -> Self
    where
        T: ToString,
    {
        self.base_uri = Some(uri.to_string());
        self
    }

    pub fn set_connect_src<T>(mut self, src: T) -> Self
    where
        T: ToString,
    {
        self.connect_src = Some(src.to_string());
        self
    }

    pub fn set_default_src<T>(mut self, src: T) -> Self
    where
        T: ToString,
    {
        self.default_src = Some(src.to_string());
        self
    }

    pub fn set_font_src<T>(mut self, src: T) -> Self
    where
        T: ToString,
    {
        self.font_src = Some(src.to_string());
        self
    }

    pub fn set_frame_src<T>(mut self, src: T) -> Self
    where
        T: ToString,
    {
        self.frame_src = Some(src.to_string());
        self
    }

    pub fn set_img_src<T>(mut self, src: T) -> Self
    where
        T: ToString,
    {
        self.img_src = Some(src.to_string());
        self
    }

    pub fn set_media_src<T>(mut self, src: T) -> Self
    where
        T: ToString,
    {
        self.media_src = Some(src.to_string());
        self
    }

    pub fn set_object_src<T>(mut self, src: T) -> Self
    where
        T: ToString,
    {
        self.object_src = Some(src.to_string());
        self
    }

    pub fn set_script_src<T>(mut self, src: T) -> Self
    where
        T: ToString,
    {
        self.script_src = Some(src.to_string());
        self
    }

    pub fn set_style_src<T>(mut self, src: T) -> Self
    where
        T: ToString,
    {
        self.style_src = Some(src.to_string());
        self
    }

    pub fn set_worker_src<T>(mut self, src: T) -> Self
    where
        T: ToString,
    {
        self.worker_src = Some(src.to_string());
        self
    }

    // Build the final CSP header value

    pub fn build_header_value(&self) -> String {
        let mut directives = Vec::new();

        if let Some(ref v) = self.base_uri {
            directives.push(format!("base-uri {}", v));
        }
        if let Some(ref v) = self.connect_src {
            directives.push(format!("connect-src {}", v));
        }
        if let Some(ref v) = self.default_src {
            directives.push(format!("default-src {}", v));
        }
        if let Some(ref v) = self.font_src {
            directives.push(format!("font-src {}", v));
        }
        if let Some(ref v) = self.frame_src {
            directives.push(format!("frame-src {}", v));
        }
        if let Some(ref v) = self.img_src {
            directives.push(format!("img-src {}", v));
        }
        if let Some(ref v) = self.media_src {
            directives.push(format!("media-src {}", v));
        }
        if let Some(ref v) = self.object_src {
            directives.push(format!("object-src {}", v));
        }
        if let Some(ref v) = self.script_src {
            directives.push(format!("script-src {}", v));
        }
        if let Some(ref v) = self.style_src {
            directives.push(format!("style-src {}", v));
        }
        if let Some(ref v) = self.worker_src {
            directives.push(format!("worker-src {}", v));
        }

        directives.join("; ")
    }
}

impl<S> Layer<S> for ContentSecurityPolicyLayer {
    type Service = ContentSecurityPolicy<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ContentSecurityPolicy {
            inner,
            layer: self.clone(),
        }
    }
}

#[derive(Clone)]
pub struct ContentSecurityPolicy<S> {
    inner: S,
    layer: ContentSecurityPolicyLayer,
}

impl<S> ContentSecurityPolicy<S> {
    pub fn new(inner: S) -> Self {
        Self {
            inner,
            layer: ContentSecurityPolicyLayer::new(),
        }
    }

    pub fn layer() -> ContentSecurityPolicyLayer {
        ContentSecurityPolicyLayer::new()
    }

    pub fn set_base_uri<T>(self, uri: T) -> Self
    where
        T: ToString,
    {
        self.map_layer(|layer| layer.set_base_uri(uri))
    }

    pub fn set_connect_src<T>(self, src: T) -> Self
    where
        T: ToString,
    {
        self.map_layer(|layer| layer.set_connect_src(src))
    }

    pub fn set_default_src<T>(self, src: T) -> Self
    where
        T: ToString,
    {
        self.map_layer(|layer| layer.set_default_src(src))
    }

    pub fn set_font_src<T>(self, src: T) -> Self
    where
        T: ToString,
    {
        self.map_layer(|layer| layer.set_font_src(src))
    }

    pub fn set_frame_src<T>(self, src: T) -> Self
    where
        T: ToString,
    {
        self.map_layer(|layer| layer.set_frame_src(src))
    }

    pub fn set_img_src<T>(self, src: T) -> Self
    where
        T: ToString,
    {
        self.map_layer(|layer| layer.set_img_src(src))
    }

    pub fn set_media_src<T>(self, src: T) -> Self
    where
        T: ToString,
    {
        self.map_layer(|layer| layer.set_media_src(src))
    }

    pub fn set_object_src<T>(self, src: T) -> Self
    where
        T: ToString,
    {
        self.map_layer(|layer| layer.set_object_src(src))
    }

    pub fn set_script_src<T>(self, src: T) -> Self
    where
        T: ToString,
    {
        self.map_layer(|layer| layer.set_script_src(src))
    }

    pub fn set_style_src<T>(self, src: T) -> Self
    where
        T: ToString,
    {
        self.map_layer(|layer| layer.set_style_src(src))
    }

    pub fn set_worker_src<T>(self, src: T) -> Self
    where
        T: ToString,
    {
        self.map_layer(|layer| layer.set_worker_src(src))
    }

    fn map_layer<F>(mut self, f: F) -> Self
    where
        F: FnOnce(ContentSecurityPolicyLayer) -> ContentSecurityPolicyLayer,
    {
        self.layer = f(self.layer);
        self
    }
}

impl<S, ReqBody> Service<Request<ReqBody>> for ContentSecurityPolicy<S>
where
    S: Service<Request<ReqBody>, Response = Response<Body>> + Clone + Send + 'static,
    ReqBody: Send + 'static,
    <S as Service<Request<ReqBody>>>::Future: Send + 'static,
{
    type Response = Response<Body>;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let layer = self.layer.clone();
        let fut = self.inner.call(req);
        Box::pin(async move {
            let mut res = fut.await?;
            res.headers_mut().insert(
                HeaderName::from_static("content-security-policy"),
                HeaderValue::from_str(&layer.build_header_value()).unwrap(),
            );
            Ok(res)
        })
    }
}
