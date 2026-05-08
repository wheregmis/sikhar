#[cfg(not(target_arch = "wasm32"))]
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

#[cfg(all(
    feature = "hotpatch",
    any(target_os = "macos", target_os = "windows", target_os = "linux")
))]
use {
    dioxus_devtools_types::DevserverMsg,
    std::{
        io::{Read, Write},
        net::TcpStream,
        sync::Once,
        thread,
        time::{Duration, Instant},
    },
};

pub(crate) fn call<O>(f: impl FnMut() -> O) -> O {
    #[cfg(all(
        feature = "hotpatch",
        any(target_os = "macos", target_os = "windows", target_os = "linux")
    ))]
    {
        subsecond::call(f)
    }

    #[cfg(not(all(
        feature = "hotpatch",
        any(target_os = "macos", target_os = "windows", target_os = "linux")
    )))]
    {
        let mut f = f;
        f()
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Default)]
pub(crate) struct HotpatchSignal {
    pending: Arc<AtomicBool>,
}

#[cfg(not(target_arch = "wasm32"))]
impl HotpatchSignal {
    pub(crate) fn register(&self, wake: impl Fn() + Send + Sync + 'static) {
        #[cfg(all(
            feature = "hotpatch",
            any(target_os = "macos", target_os = "windows", target_os = "linux")
        ))]
        {
            let pending = Arc::clone(&self.pending);
            subsecond::register_handler(Arc::new(move || {
                pending.store(true, Ordering::SeqCst);
                wake();
            }));
            connect_once();
        }

        #[cfg(not(all(
            feature = "hotpatch",
            any(target_os = "macos", target_os = "windows", target_os = "linux")
        )))]
        {
            let _ = wake;
        }
    }

    pub(crate) fn take_pending(&self) -> bool {
        self.pending.swap(false, Ordering::SeqCst)
    }

    #[cfg(test)]
    fn mark_pending_for_test(&self) {
        self.pending.store(true, Ordering::SeqCst);
    }
}

#[cfg(any(
    test,
    all(
        feature = "hotpatch",
        any(target_os = "macos", target_os = "windows", target_os = "linux")
    )
))]
fn should_connect_to_devserver(cli_enabled: bool, endpoint: Option<&str>) -> Option<String> {
    if cli_enabled {
        endpoint.map(str::to_owned)
    } else {
        None
    }
}

#[cfg(any(
    test,
    all(
        feature = "hotpatch",
        any(target_os = "macos", target_os = "windows", target_os = "linux")
    )
))]
fn hotreload_matches_target(
    for_pid: Option<u32>,
    for_build_id: Option<u64>,
    build_id: u64,
) -> bool {
    let pid_matches = for_pid.is_none_or(|pid| pid == std::process::id());
    let build_matches = for_build_id.is_none_or(|id| build_id == 0 || id == build_id);
    pid_matches && build_matches
}

#[cfg(all(
    feature = "hotpatch",
    any(target_os = "macos", target_os = "windows", target_os = "linux")
))]
static CONNECT_ONCE: Once = Once::new();

#[cfg(all(
    feature = "hotpatch",
    any(target_os = "macos", target_os = "windows", target_os = "linux")
))]
fn connect_once() {
    CONNECT_ONCE.call_once(|| {
        let Some(endpoint) = should_connect_to_devserver(
            dioxus_cli_config::is_cli_enabled(),
            dioxus_cli_config::devserver_ws_endpoint().as_deref(),
        ) else {
            return;
        };

        thread::spawn(move || {
            run_connection_loop(
                Duration::from_millis(500),
                || true,
                || run_connection(&endpoint),
            );
        });
    });
}

#[cfg(all(
    feature = "hotpatch",
    any(target_os = "macos", target_os = "windows", target_os = "linux")
))]
fn run_connection_loop<F, K>(retry_delay: Duration, mut keep_running: K, mut run_once: F)
where
    F: FnMut() -> Result<(), String>,
    K: FnMut() -> bool,
{
    while keep_running() {
        if let Err(err) = run_once() {
            log::warn!("subsecond hotpatch connection failed: {err}");
        }
        if retry_delay > Duration::ZERO {
            thread::sleep(retry_delay);
        }
    }
}

#[cfg(all(
    feature = "hotpatch",
    any(target_os = "macos", target_os = "windows", target_os = "linux")
))]
fn run_connection(endpoint: &str) -> Result<(), String> {
    let request_url = hotpatch_request_url(endpoint);
    let url = SplitWsUrl::parse(&request_url)?;
    let mut stream = TcpStream::connect(format!("{}:{}", url.host, url.port))
        .map_err(|err| format!("failed to connect to Dioxus devserver: {err}"))?;
    let _ = stream.set_nodelay(true);
    let _ = stream.set_read_timeout(Some(Duration::from_millis(50)));
    let _ = stream.set_write_timeout(Some(Duration::from_secs(5)));

    write_handshake(&mut stream, &url)?;
    let leftover = read_handshake_response(&mut stream)?;

    let mut parser = WebSocketParser::default();
    if !leftover.is_empty() && parse_incoming(&mut parser, &mut stream, &leftover)? {
        return Ok(());
    }

    loop {
        let mut buffer = [0u8; 65535];
        match stream.read(&mut buffer) {
            Ok(0) => return Err("Dioxus devserver websocket closed".to_string()),
            Ok(bytes_read) => {
                if parse_incoming(&mut parser, &mut stream, &buffer[..bytes_read])? {
                    return Ok(());
                }
            }
            Err(err)
                if matches!(
                    err.kind(),
                    std::io::ErrorKind::WouldBlock
                        | std::io::ErrorKind::TimedOut
                        | std::io::ErrorKind::Interrupted
                ) => {}
            Err(err) => return Err(format!("failed to read Dioxus devserver websocket: {err}")),
        }
    }
}

#[cfg(all(
    feature = "hotpatch",
    any(target_os = "macos", target_os = "windows", target_os = "linux")
))]
fn hotpatch_request_url(endpoint: &str) -> String {
    let separator = if endpoint.contains('?') { '&' } else { '?' };
    format!(
        "{endpoint}{separator}aslr_reference={}&build_id={}&pid={}",
        subsecond::aslr_reference(),
        dioxus_cli_config::build_id(),
        std::process::id()
    )
}

#[cfg(all(
    feature = "hotpatch",
    any(target_os = "macos", target_os = "windows", target_os = "linux")
))]
fn write_handshake(stream: &mut TcpStream, url: &SplitWsUrl) -> Result<(), String> {
    let host_header = if url.port == "80" {
        url.host.clone()
    } else {
        format!("{}:{}", url.host, url.port)
    };
    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: Upgrade\r\nUpgrade: websocket\r\nSec-WebSocket-Version: 13\r\nSec-WebSocket-Key: c3BhcnNoYS1ob3RwYXRjaA==\r\n\r\n",
        url.path, host_header
    );
    stream
        .write_all(request.as_bytes())
        .map_err(|err| format!("failed to write websocket handshake: {err}"))?;
    stream
        .flush()
        .map_err(|err| format!("failed to flush websocket handshake: {err}"))
}

#[cfg(all(
    feature = "hotpatch",
    any(target_os = "macos", target_os = "windows", target_os = "linux")
))]
fn read_handshake_response(stream: &mut TcpStream) -> Result<Vec<u8>, String> {
    let start = Instant::now();
    let mut buffer = Vec::new();
    let mut chunk = [0u8; 1024];

    loop {
        if let Some(end) = find_header_terminator(&buffer) {
            let headers = String::from_utf8_lossy(&buffer[..end]);
            let first_line = headers.lines().next().unwrap_or_default();
            if !first_line.contains("101") {
                return Err(format!("websocket upgrade rejected: {first_line}"));
            }
            return Ok(buffer[end + 4..].to_vec());
        }

        if start.elapsed() > Duration::from_secs(5) {
            return Err("timeout waiting for websocket upgrade response".to_string());
        }

        match stream.read(&mut chunk) {
            Ok(0) => return Err("connection closed during websocket handshake".to_string()),
            Ok(bytes_read) => buffer.extend_from_slice(&chunk[..bytes_read]),
            Err(err)
                if matches!(
                    err.kind(),
                    std::io::ErrorKind::WouldBlock
                        | std::io::ErrorKind::TimedOut
                        | std::io::ErrorKind::Interrupted
                ) => {}
            Err(err) => return Err(format!("failed to read websocket handshake: {err}")),
        }
    }
}

#[cfg(all(
    feature = "hotpatch",
    any(target_os = "macos", target_os = "windows", target_os = "linux")
))]
fn find_header_terminator(buffer: &[u8]) -> Option<usize> {
    buffer.windows(4).position(|window| window == b"\r\n\r\n")
}

#[cfg(all(
    feature = "hotpatch",
    any(target_os = "macos", target_os = "windows", target_os = "linux")
))]
fn parse_incoming(
    parser: &mut WebSocketParser,
    stream: &mut TcpStream,
    data: &[u8],
) -> Result<bool, String> {
    let mut disconnected = false;
    for message in parser.push(data)? {
        match message {
            WebSocketMessage::Text(text) => handle_text_message(&text),
            WebSocketMessage::Ping(payload) => write_client_frame(stream, 0xA, &payload)?,
            WebSocketMessage::Pong | WebSocketMessage::Binary(_) => {}
            WebSocketMessage::Close => disconnected = true,
        }
    }
    Ok(disconnected)
}

#[cfg(all(
    feature = "hotpatch",
    any(target_os = "macos", target_os = "windows", target_os = "linux")
))]
fn handle_text_message(text: &str) {
    let Ok(msg) = serde_json::from_str::<DevserverMsg>(text) else {
        return;
    };

    let DevserverMsg::HotReload(msg) = msg else {
        return;
    };
    if !hotreload_matches_target(msg.for_pid, msg.for_build_id, dioxus_cli_config::build_id()) {
        return;
    }
    let Some(jump_table) = msg.jump_table else {
        return;
    };

    if let Err(err) = unsafe { subsecond::apply_patch(jump_table) } {
        log::warn!("failed to apply Subsecond hotpatch: {err}");
    }
}

#[cfg(all(
    feature = "hotpatch",
    any(target_os = "macos", target_os = "windows", target_os = "linux")
))]
fn write_client_frame(stream: &mut TcpStream, opcode: u8, payload: &[u8]) -> Result<(), String> {
    let mut frame = Vec::with_capacity(payload.len() + 14);
    frame.push(0x80 | (opcode & 0x0f));
    let mask = [0x13, 0x37, 0x51, 0x8d];
    match payload.len() {
        len @ 0..=125 => frame.push(0x80 | len as u8),
        len @ 126..=65535 => {
            frame.push(0x80 | 126);
            frame.extend_from_slice(&(len as u16).to_be_bytes());
        }
        len => {
            frame.push(0x80 | 127);
            frame.extend_from_slice(&(len as u64).to_be_bytes());
        }
    }
    frame.extend_from_slice(&mask);
    frame.extend(
        payload
            .iter()
            .enumerate()
            .map(|(index, byte)| byte ^ mask[index % 4]),
    );
    stream
        .write_all(&frame)
        .map_err(|err| format!("failed to write websocket frame: {err}"))?;
    stream
        .flush()
        .map_err(|err| format!("failed to flush websocket frame: {err}"))
}

#[cfg(all(
    feature = "hotpatch",
    any(target_os = "macos", target_os = "windows", target_os = "linux")
))]
#[derive(Debug, PartialEq, Eq)]
struct SplitWsUrl {
    host: String,
    port: String,
    path: String,
}

#[cfg(all(
    feature = "hotpatch",
    any(target_os = "macos", target_os = "windows", target_os = "linux")
))]
impl SplitWsUrl {
    fn parse(url: &str) -> Result<Self, String> {
        let without_scheme = url
            .strip_prefix("ws://")
            .or_else(|| url.strip_prefix("http://"))
            .ok_or_else(|| format!("unsupported websocket url: {url}"))?;
        let (authority, path) = without_scheme
            .split_once('/')
            .map(|(authority, path)| (authority, format!("/{path}")))
            .unwrap_or((without_scheme, String::from("/")));
        let (host, port) = authority
            .rsplit_once(':')
            .map(|(host, port)| (host.to_owned(), port.to_owned()))
            .unwrap_or_else(|| (authority.to_owned(), String::from("80")));

        if host.is_empty() || port.is_empty() {
            return Err(format!("invalid websocket url: {url}"));
        }

        Ok(Self { host, port, path })
    }
}

#[cfg(any(
    test,
    all(
        feature = "hotpatch",
        any(target_os = "macos", target_os = "windows", target_os = "linux")
    )
))]
#[derive(Debug, PartialEq, Eq)]
enum WebSocketMessage {
    Text(String),
    Binary(Vec<u8>),
    Ping(Vec<u8>),
    Pong,
    Close,
}

#[cfg(any(
    test,
    all(
        feature = "hotpatch",
        any(target_os = "macos", target_os = "windows", target_os = "linux")
    )
))]
#[derive(Default)]
struct WebSocketParser {
    buffer: Vec<u8>,
}

#[cfg(any(
    test,
    all(
        feature = "hotpatch",
        any(target_os = "macos", target_os = "windows", target_os = "linux")
    )
))]
impl WebSocketParser {
    fn push(&mut self, data: &[u8]) -> Result<Vec<WebSocketMessage>, String> {
        self.buffer.extend_from_slice(data);
        let mut messages = Vec::new();

        loop {
            let Some((frame_len, message)) = parse_frame(&self.buffer)? else {
                break;
            };
            self.buffer.drain(..frame_len);
            messages.push(message);
        }

        Ok(messages)
    }
}

#[cfg(any(
    test,
    all(
        feature = "hotpatch",
        any(target_os = "macos", target_os = "windows", target_os = "linux")
    )
))]
fn parse_frame(buffer: &[u8]) -> Result<Option<(usize, WebSocketMessage)>, String> {
    if buffer.len() < 2 {
        return Ok(None);
    }

    let fin = buffer[0] & 0x80 != 0;
    let opcode = buffer[0] & 0x0f;
    if !fin {
        return Err("fragmented websocket frames are not supported".to_string());
    }

    let masked = buffer[1] & 0x80 != 0;
    let len_code = buffer[1] & 0x7f;
    let mut offset = 2usize;
    let payload_len = match len_code {
        0..=125 => len_code as usize,
        126 => {
            if buffer.len() < offset + 2 {
                return Ok(None);
            }
            let len = u16::from_be_bytes([buffer[offset], buffer[offset + 1]]) as usize;
            offset += 2;
            len
        }
        127 => {
            if buffer.len() < offset + 8 {
                return Ok(None);
            }
            let len = u64::from_be_bytes([
                buffer[offset],
                buffer[offset + 1],
                buffer[offset + 2],
                buffer[offset + 3],
                buffer[offset + 4],
                buffer[offset + 5],
                buffer[offset + 6],
                buffer[offset + 7],
            ]);
            offset += 8;
            usize::try_from(len).map_err(|_| "websocket frame too large".to_string())?
        }
        _ => unreachable!(),
    };

    let mask = if masked {
        if buffer.len() < offset + 4 {
            return Ok(None);
        }
        let mask = [
            buffer[offset],
            buffer[offset + 1],
            buffer[offset + 2],
            buffer[offset + 3],
        ];
        offset += 4;
        Some(mask)
    } else {
        None
    };

    let frame_len = offset
        .checked_add(payload_len)
        .ok_or_else(|| "websocket frame too large".to_string())?;
    if buffer.len() < frame_len {
        return Ok(None);
    }

    let mut payload = buffer[offset..frame_len].to_vec();
    if let Some(mask) = mask {
        for (index, byte) in payload.iter_mut().enumerate() {
            *byte ^= mask[index % 4];
        }
    }

    let message = match opcode {
        0x1 => WebSocketMessage::Text(
            String::from_utf8(payload).map_err(|_| "websocket text was not UTF-8".to_string())?,
        ),
        0x2 => WebSocketMessage::Binary(payload),
        0x8 => WebSocketMessage::Close,
        0x9 => WebSocketMessage::Ping(payload),
        0xA => WebSocketMessage::Pong,
        other => return Err(format!("unsupported websocket opcode: {other}")),
    };

    Ok(Some((frame_len, message)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text_frame(text: &str) -> Vec<u8> {
        let mut frame = vec![0x81, text.len() as u8];
        frame.extend_from_slice(text.as_bytes());
        frame
    }

    #[test]
    fn devserver_connection_requires_cli_and_endpoint() {
        assert_eq!(
            should_connect_to_devserver(true, Some("ws://127.0.0.1:8080/_dioxus")),
            Some(String::from("ws://127.0.0.1:8080/_dioxus"))
        );
        assert_eq!(should_connect_to_devserver(false, Some("ws://x")), None);
        assert_eq!(should_connect_to_devserver(true, None), None);
    }

    #[test]
    fn websocket_parser_handles_partial_text_frame() {
        let mut parser = WebSocketParser::default();
        let frame = text_frame("patch");

        assert_eq!(parser.push(&frame[..2]).unwrap(), Vec::new());
        assert_eq!(
            parser.push(&frame[2..]).unwrap(),
            vec![WebSocketMessage::Text(String::from("patch"))]
        );
    }

    #[test]
    fn websocket_parser_handles_ping_close_and_malformed_frames() {
        let mut parser = WebSocketParser::default();
        assert_eq!(
            parser.push(&[0x89, 0x02, b'o', b'k']).unwrap(),
            vec![WebSocketMessage::Ping(vec![b'o', b'k'])]
        );
        assert_eq!(
            parser.push(&[0x88, 0x00]).unwrap(),
            vec![WebSocketMessage::Close]
        );

        let mut malformed = WebSocketParser::default();
        assert!(malformed.push(&[0x01, 0x00]).is_err());
    }

    #[test]
    fn websocket_parser_unmasks_payloads() {
        let mut parser = WebSocketParser::default();
        let mask = [1u8, 2, 3, 4];
        let payload = b"masked";
        let mut frame = vec![0x81, 0x80 | payload.len() as u8];
        frame.extend_from_slice(&mask);
        frame.extend(
            payload
                .iter()
                .enumerate()
                .map(|(index, byte)| byte ^ mask[index % 4]),
        );

        assert_eq!(
            parser.push(&frame).unwrap(),
            vec![WebSocketMessage::Text(String::from("masked"))]
        );
    }

    #[test]
    fn hotreload_target_filter_accepts_absent_or_matching_pid() {
        let this_pid = std::process::id();
        assert!(hotreload_matches_target(None, None, 0));
        assert!(hotreload_matches_target(Some(this_pid), None, 0));
        assert!(!hotreload_matches_target(
            Some(this_pid.wrapping_add(1)),
            None,
            0
        ));
        assert!(hotreload_matches_target(None, Some(7), 0));
        assert!(hotreload_matches_target(None, Some(7), 7));
        assert!(!hotreload_matches_target(None, Some(7), 8));
    }

    #[test]
    fn pending_flag_is_consumed_once() {
        let signal = HotpatchSignal::default();
        assert!(!signal.take_pending());

        signal.mark_pending_for_test();
        assert!(signal.take_pending());
        assert!(!signal.take_pending());
    }

    #[cfg(all(
        feature = "hotpatch",
        any(target_os = "macos", target_os = "windows", target_os = "linux")
    ))]
    #[test]
    fn split_websocket_url_keeps_path_and_query() {
        assert_eq!(
            SplitWsUrl::parse("ws://127.0.0.1:8080/_dioxus?pid=1").unwrap(),
            SplitWsUrl {
                host: String::from("127.0.0.1"),
                port: String::from("8080"),
                path: String::from("/_dioxus?pid=1"),
            }
        );
    }
}
