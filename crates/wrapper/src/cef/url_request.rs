use crate::{
    ref_counted_ptr, AuthCallback, CefString, ErrorCode, RefCountedPtr, Request, RequestContext,
    Response, Wrappable, Wrapped
};
use bindings::{
    cef_auth_callback_t, cef_string_t, cef_urlrequest_client_t, cef_urlrequest_create,
    cef_urlrequest_status_t, cef_urlrequest_t
};
use std::{
    ffi::{c_int, c_void},
    mem::zeroed,
    ptr::null_mut,
    slice::from_raw_parts
};

/// Flags that represent CefURLRequest status.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UrlRequestStatus {
    /// Unknown status.
    Unknown = 0,

    /// Request succeeded.
    Success,

    /// An IO request is pending, and the caller will be informed when it is
    /// completed.
    IoPending,

    /// Request was canceled programatically.
    Canceled,

    /// Request failed for some reason.
    Failed
}

impl From<cef_urlrequest_status_t> for UrlRequestStatus {
    fn from(value: cef_urlrequest_status_t) -> Self {
        Self::from(&value)
    }
}

impl From<&cef_urlrequest_status_t> for UrlRequestStatus {
    fn from(value: &cef_urlrequest_status_t) -> Self {
        match value {
            cef_urlrequest_status_t::UR_UNKNOWN => Self::Unknown,
            cef_urlrequest_status_t::UR_SUCCESS => Self::Success,
            cef_urlrequest_status_t::UR_IO_PENDING => Self::IoPending,
            cef_urlrequest_status_t::UR_CANCELED => Self::Canceled,
            cef_urlrequest_status_t::UR_FAILED => Self::Failed
        }
    }
}

impl From<UrlRequestStatus> for cef_urlrequest_status_t {
    fn from(value: UrlRequestStatus) -> Self {
        Self::from(&value)
    }
}

impl From<&UrlRequestStatus> for cef_urlrequest_status_t {
    fn from(value: &UrlRequestStatus) -> Self {
        match value {
            UrlRequestStatus::Unknown => cef_urlrequest_status_t::UR_UNKNOWN,
            UrlRequestStatus::Success => cef_urlrequest_status_t::UR_SUCCESS,
            UrlRequestStatus::IoPending => cef_urlrequest_status_t::UR_IO_PENDING,
            UrlRequestStatus::Canceled => cef_urlrequest_status_t::UR_CANCELED,
            UrlRequestStatus::Failed => cef_urlrequest_status_t::UR_FAILED
        }
    }
}

// Structure used to make a URL request. URL requests are not associated with a
// browser instance so no cef_client_t callbacks will be executed. URL requests
// can be created on any valid CEF thread in either the browser or render
// process. Once created the functions of the URL request object must be
// accessed on the same thread that created it.
ref_counted_ptr!(UrlRequest, cef_urlrequest_t);

impl UrlRequest {
    /// Create a new URL request that is not associated with a specific browser or
    /// frame. Use cef_frame_t::CreateURLRequest instead if you want the request to
    /// have this association, in which case it may be handled differently (see
    /// documentation on that function). A request created with this function may
    /// only originate from the browser process, and will behave as follows:
    ///   - It may be intercepted by the client via CefResourceRequestHandler or
    ///     CefSchemeHandlerFactory.
    ///   - POST data may only contain only a single element of type PDE_TYPE_FILE
    ///     or PDE_TYPE_BYTES.
    ///   - If |request_context| is empty the global request context will be used.
    ///
    /// The |request| object will be marked as read-only after calling this
    /// function.
    pub fn new(
        request: Request,
        client: UrlRequestClient,
        request_context: Option<RequestContext>
    ) -> Self {
        unsafe {
            let request_context = request_context
                .map(|request_context| request_context.into_raw())
                .unwrap_or(null_mut());

            Self::from_ptr_unchecked(cef_urlrequest_create(
                request.into_raw(),
                client.into_raw(),
                request_context
            ))
        }
    }

    /// Returns the request object used to create this URL request. The returned
    /// object is read-only and should not be modified.
    pub fn get_request(&self) -> Option<Request> {
        self.0
            .get_request
            .map(|get_request| unsafe { Request::from_ptr_unchecked(get_request(self.as_ptr())) })
    }

    /// Returns the client.
    pub fn get_client(&self) -> Option<UrlRequestClient> {
        self.0
            .get_client
            .map(|get_client| unsafe {
                UrlRequestClient::from_ptr_unchecked(get_client(self.as_ptr()))
            })
    }

    /// Returns the request status.
    pub fn get_request_status(&self) -> Option<UrlRequestStatus> {
        self.0
            .get_request_status
            .map(|get_request_status| unsafe { get_request_status(self.as_ptr()).into() })
    }

    /// Returns the request error if status is UR_CANCELED or UR_FAILED, or 0
    /// otherwise.
    pub fn get_request_error(&self) -> Option<ErrorCode> {
        self.0
            .get_request_error
            .map(|get_request_error| unsafe { get_request_error(self.as_ptr()).into() })
    }

    /// Returns the response, or NULL if no response information is available.
    /// Response information will only be available after the upload has
    /// completed. The returned object is read-only and should not be modified.
    pub fn get_response(&self) -> Option<Response> {
        self.0
            .get_response
            .and_then(|get_response| unsafe { Response::from_ptr(get_response(self.as_ptr())) })
    }

    /// Returns true (1) if the response body was served from the cache. This
    /// includes responses for which revalidation was required.
    pub fn response_was_cached(&self) -> bool {
        self.0
            .response_was_cached
            .map(|response_was_cached| unsafe { response_was_cached(self.as_ptr()) != 0 })
            .unwrap_or(false)
    }

    /// Cancel the request.
    pub fn cancel(&self) {
        self.0
            .cancel
            .map(|cancel| unsafe { cancel(self.as_ptr()) });
    }
}

/// Structure that should be implemented by the cef_urlrequest_t client. The
/// functions of this structure will be called on the same thread that created
/// the request unless otherwise documented.
#[allow(unused_variables)]
pub trait UrlRequestClientCallbacks: Send + Sync + 'static {
    /// Notifies the client that the request has completed. Use the
    /// cef_urlrequest_t::GetRequestStatus function to determine if the request
    /// was successful or not.
    fn on_request_complete(&self, request: UrlRequest) {}

    /// Notifies the client of upload progress. |current| denotes the number of
    /// bytes sent so far and |total| is the total size of uploading data (or -1
    /// if chunked upload is enabled). This function will only be called if the
    /// UR_FLAG_REPORT_UPLOAD_PROGRESS flag is set on the request.
    fn on_upload_progress(&self, request: UrlRequest, current: i64, total: i64) {}

    /// Notifies the client of download progress. |current| denotes the number of
    /// bytes received up to the call and |total| is the expected total size of
    /// the response (or -1 if not determined).
    fn on_download_progress(&self, request: UrlRequest, current: i64, total: i64) {}

    /// Called when some part of the response is read. |data| contains the current
    /// bytes received since the last call. This function will not be called if
    /// the UR_FLAG_NO_DOWNLOAD_DATA flag is set on the request.
    fn on_download_data(&self, request: UrlRequest, data: &[u8]) {}

    /// Called on the IO thread when the browser needs credentials from the user.
    /// |isProxy| indicates whether the host is a proxy server. |host| contains
    /// the hostname and |port| contains the port number. Return true (1) to
    /// continue the request and call cef_auth_callback_t::cont() when the
    /// authentication information is available. If the request has an associated
    /// browser/frame then returning false (0) will result in a call to
    /// GetAuthCredentials on the cef_request_handler_t associated with that
    /// browser, if any. Otherwise, returning false (0) will cancel the request
    /// immediately. This function will only be called for requests initiated from
    /// the browser process.
    fn get_auth_credentials(
        &self,
        is_proxy: bool,
        host: &str,
        port: u16,
        realm: &str,
        scheme: &str,
        callback: AuthCallback
    ) -> bool {
        false
    }
}

// Structure that should be implemented by the cef_urlrequest_t client. The
// functions of this structure will be called on the same thread that created
// the request unless otherwise documented.
ref_counted_ptr!(UrlRequestClient, cef_urlrequest_client_t);

impl UrlRequestClient {
    pub fn new<C: UrlRequestClientCallbacks>(delegate: C) -> Self {
        Self(UrlRequestClientWrapper::new(delegate).wrap())
    }
}

/// Translates CEF -> Rust callbacks.
struct UrlRequestClientWrapper(Box<dyn UrlRequestClientCallbacks>);

impl UrlRequestClientWrapper {
    pub fn new<C: UrlRequestClientCallbacks>(delegate: C) -> Self {
        Self(Box::new(delegate))
    }

    /// Notifies the client that the request has completed. Use the
    /// cef_urlrequest_t::GetRequestStatus function to determine if the request
    /// was successful or not.
    unsafe extern "C" fn c_on_request_complete(
        this: *mut cef_urlrequest_client_t,
        request: *mut cef_urlrequest_t
    ) {
        let this: &Self = Wrapped::wrappable(this);
        let request = UrlRequest::from_ptr_unchecked(request);

        this.0.on_request_complete(request);
    }

    /// Notifies the client of upload progress. |current| denotes the number of
    /// bytes sent so far and |total| is the total size of uploading data (or -1
    /// if chunked upload is enabled). This function will only be called if the
    /// UR_FLAG_REPORT_UPLOAD_PROGRESS flag is set on the request.
    unsafe extern "C" fn c_on_upload_progress(
        this: *mut cef_urlrequest_client_t,
        request: *mut cef_urlrequest_t,
        current: i64,
        total: i64
    ) {
        let this: &Self = Wrapped::wrappable(this);
        let request = UrlRequest::from_ptr_unchecked(request);

        this.0
            .on_upload_progress(request, current, total);
    }

    /// Notifies the client of download progress. |current| denotes the number of
    /// bytes received up to the call and |total| is the expected total size of
    /// the response (or -1 if not determined).
    unsafe extern "C" fn c_on_download_progress(
        this: *mut cef_urlrequest_client_t,
        request: *mut cef_urlrequest_t,
        current: i64,
        total: i64
    ) {
        let this: &Self = Wrapped::wrappable(this);
        let request = UrlRequest::from_ptr_unchecked(request);

        this.0
            .on_download_progress(request, current, total);
    }

    /// Called when some part of the response is read. |data| contains the current
    /// bytes received since the last call. This function will not be called if
    /// the UR_FLAG_NO_DOWNLOAD_DATA flag is set on the request.
    unsafe extern "C" fn c_on_download_data(
        this: *mut cef_urlrequest_client_t,
        request: *mut cef_urlrequest_t,
        data: *const c_void,
        data_length: usize
    ) {
        let this: &Self = Wrapped::wrappable(this);
        let request = UrlRequest::from_ptr_unchecked(request);
        let data = from_raw_parts(data as *const u8, data_length);

        this.0
            .on_download_data(request, data);
    }

    /// Called on the IO thread when the browser needs credentials from the user.
    /// |isProxy| indicates whether the host is a proxy server. |host| contains
    /// the hostname and |port| contains the port number. Return true (1) to
    /// continue the request and call cef_auth_callback_t::cont() when the
    /// authentication information is available. If the request has an associated
    /// browser/frame then returning false (0) will result in a call to
    /// GetAuthCredentials on the cef_request_handler_t associated with that
    /// browser, if any. Otherwise, returning false (0) will cancel the request
    /// immediately. This function will only be called for requests initiated from
    /// the browser process.
    unsafe extern "C" fn c_get_auth_credentials(
        this: *mut cef_urlrequest_client_t,
        is_proxy: c_int,
        host: *const cef_string_t,
        port: c_int,
        realm: *const cef_string_t,
        scheme: *const cef_string_t,
        callback: *mut cef_auth_callback_t
    ) -> c_int {
        let this: &Self = Wrapped::wrappable(this);
        let host: String = CefString::from_ptr_unchecked(host).into();
        let realm: String = CefString::from_ptr_unchecked(realm).into();
        let scheme: String = CefString::from_ptr_unchecked(scheme).into();
        let callback = AuthCallback::from_ptr_unchecked(callback);

        this.0
            .get_auth_credentials(is_proxy != 0, &host, port as u16, &realm, &scheme, callback)
            as c_int
    }
}

impl Wrappable for UrlRequestClientWrapper {
    type Cef = cef_urlrequest_client_t;

    /// Converts this to a smart pointer.
    fn wrap(self) -> RefCountedPtr<cef_urlrequest_client_t> {
        RefCountedPtr::wrap(
            cef_urlrequest_client_t {
                base:                 unsafe { zeroed() },
                on_request_complete:  Some(Self::c_on_request_complete),
                on_upload_progress:   Some(Self::c_on_upload_progress),
                on_download_progress: Some(Self::c_on_download_progress),
                on_download_data:     Some(Self::c_on_download_data),
                get_auth_credentials: Some(Self::c_get_auth_credentials)
            },
            self
        )
    }
}
