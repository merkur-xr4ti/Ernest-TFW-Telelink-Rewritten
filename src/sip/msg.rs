//! Small SIP toolkit: parsing and building messages, digest authentication, SDP helpers.

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub struct SipMsg {
    pub start: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

fn canonical(name: &str) -> String {
    let n = name.trim().to_ascii_lowercase();
    match n.as_str() {
        "v" => "via".into(),
        "f" => "from".into(),
        "t" => "to".into(),
        "i" => "call-id".into(),
        "m" => "contact".into(),
        "l" => "content-length".into(),
        "c" => "content-type".into(),
        "k" => "supported".into(),
        "e" => "content-encoding".into(),
        "s" => "subject".into(),
        _ => n,
    }
}

/// Split a header value on commas that are not inside quotes or angle brackets.
pub fn split_list(v: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_q = false;
    let mut depth = 0i32;
    for ch in v.chars() {
        match ch {
            '"' => {
                in_q = !in_q;
                cur.push(ch)
            }
            '<' if !in_q => {
                depth += 1;
                cur.push(ch)
            }
            '>' if !in_q => {
                depth -= 1;
                cur.push(ch)
            }
            ',' if !in_q && depth <= 0 => {
                if !cur.trim().is_empty() {
                    out.push(cur.trim().to_string());
                }
                cur.clear();
            }
            _ => cur.push(ch),
        }
    }
    if !cur.trim().is_empty() {
        out.push(cur.trim().to_string());
    }
    out
}

impl SipMsg {
    pub fn parse(data: &[u8]) -> Option<SipMsg> {
        let split = data.windows(4).position(|w| w == b"\r\n\r\n");
        let (head, rest) = match split {
            Some(p) => (&data[..p], &data[p + 4..]),
            None => (data, &data[data.len()..]),
        };
        let head = String::from_utf8_lossy(head);
        let mut lines: Vec<String> = Vec::new();
        for line in head.split("\r\n").flat_map(|l| l.split('\n')) {
            if (line.starts_with(' ') || line.starts_with('\t')) && !lines.is_empty() {
                let last = lines.last_mut().unwrap();
                last.push(' ');
                last.push_str(line.trim());
            } else {
                lines.push(line.to_string());
            }
        }
        let mut it = lines.into_iter();
        let start = it.next()?.trim().to_string();
        if !(start.starts_with("SIP/2.0 ") || start.ends_with(" SIP/2.0")) {
            return None;
        }
        let mut headers = Vec::new();
        for l in it {
            if let Some(p) = l.find(':') {
                headers.push((canonical(&l[..p]), l[p + 1..].trim().to_string()));
            }
        }
        let mut msg = SipMsg { start, headers, body: Vec::new() };
        let len = msg
            .header("content-length")
            .and_then(|v| v.trim().parse::<usize>().ok())
            .unwrap_or(rest.len());
        msg.body = rest[..len.min(rest.len())].to_vec();
        Some(msg)
    }

    pub fn is_request(&self) -> bool {
        !self.start.starts_with("SIP/2.0 ")
    }
    pub fn method(&self) -> String {
        if self.is_request() {
            self.start.split_whitespace().next().unwrap_or("").to_ascii_uppercase()
        } else {
            self.cseq().1
        }
    }
    pub fn status(&self) -> u16 {
        if self.is_request() {
            0
        } else {
            self.start.split_whitespace().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0)
        }
    }
    pub fn reason(&self) -> String {
        self.start.splitn(3, ' ').nth(2).unwrap_or("").to_string()
    }
    pub fn header(&self, name: &str) -> Option<&str> {
        let n = canonical(name);
        self.headers.iter().find(|(k, _)| *k == n).map(|(_, v)| v.as_str())
    }
    /// Every value of a header, with comma-separated lists split.
    pub fn header_all(&self, name: &str) -> Vec<String> {
        let n = canonical(name);
        self.headers
            .iter()
            .filter(|(k, _)| *k == n)
            .flat_map(|(_, v)| split_list(v))
            .collect()
    }
    pub fn cseq(&self) -> (u32, String) {
        let v = self.header("cseq").unwrap_or("");
        let mut p = v.split_whitespace();
        let n = p.next().and_then(|s| s.parse().ok()).unwrap_or(0);
        let m = p.next().unwrap_or("").to_ascii_uppercase();
        (n, m)
    }
    pub fn call_id(&self) -> String {
        self.header("call-id").unwrap_or("").to_string()
    }
}

/// Value of a `;name=value` parameter in a header.
pub fn param(h: &str, name: &str) -> Option<String> {
    // Only look at parameters outside the <...> part.
    let tail = match h.rfind('>') {
        Some(p) => &h[p + 1..],
        None => h,
    };
    for part in tail.split(';').skip(if h.contains('>') { 0 } else { 1 }) {
        let mut kv = part.splitn(2, '=');
        let k = kv.next().unwrap_or("").trim();
        if k.eq_ignore_ascii_case(name) {
            return Some(kv.next().unwrap_or("").trim().trim_matches('"').to_string());
        }
    }
    None
}

pub fn tag_of(h: &str) -> Option<String> {
    param(h, "tag").filter(|t| !t.is_empty())
}

/// The URI inside a name-addr (`"Bob" <sip:bob@x>;tag=1` -> `sip:bob@x`).
pub fn uri_of(h: &str) -> String {
    if let (Some(a), Some(b)) = (h.find('<'), h.find('>')) {
        if b > a {
            return h[a + 1..b].to_string();
        }
    }
    h.split(';').next().unwrap_or("").trim().to_string()
}



static COUNTER: AtomicU64 = AtomicU64::new(1);

pub fn random_hex(len: usize) -> String {
    let mut out = String::new();
    while out.len() < len {
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let t = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0);
        let d = md5::compute(format!("{t}-{n}-{}", std::process::id()));
        out.push_str(&format!("{:x}", d));
    }
    out.truncate(len);
    out
}

pub fn new_branch() -> String {
    format!("z9hG4bK{}", random_hex(16))
}

/// A digest challenge from WWW-Authenticate / Proxy-Authenticate.
#[derive(Clone, Debug, Default)]
pub struct Challenge {
    pub proxy: bool,
    pub realm: String,
    pub nonce: String,
    pub opaque: Option<String>,
    pub algorithm: Option<String>,
    pub qop_auth: bool,
    pub nc: u32,
}

impl Challenge {
    pub fn from_response(msg: &SipMsg) -> Option<Challenge> {
        let (v, proxy) = if let Some(v) = msg.header("www-authenticate") {
            (v, false)
        } else {
            (msg.header("proxy-authenticate")?, true)
        };
        let v = v.trim();
        if !v.to_ascii_lowercase().starts_with("digest") {
            return None;
        }
        let mut c = Challenge { proxy, ..Default::default() };
        for part in split_list(&v[6..]) {
            let mut kv = part.splitn(2, '=');
            let k = kv.next().unwrap_or("").trim().to_ascii_lowercase();
            let val = kv.next().unwrap_or("").trim().trim_matches('"').to_string();
            match k.as_str() {
                "realm" => c.realm = val,
                "nonce" => c.nonce = val,
                "opaque" => c.opaque = Some(val),
                "algorithm" => c.algorithm = Some(val),
                "qop" => c.qop_auth = val.split(',').any(|q| q.trim().eq_ignore_ascii_case("auth")),
                _ => {}
            }
        }
        Some(c)
    }

    /// Build the (header name, header value) answering this challenge.
    pub fn answer(&mut self, user: &str, password: &str, method: &str, uri: &str) -> (String, String) {
        let ha1 = format!("{:x}", md5::compute(format!("{user}:{}:{password}", self.realm)));
        let ha2 = format!("{:x}", md5::compute(format!("{method}:{uri}")));
        let mut v = format!(
            "Digest username=\"{user}\", realm=\"{}\", nonce=\"{}\", uri=\"{uri}\"",
            self.realm, self.nonce
        );
        if self.qop_auth {
            self.nc += 1;
            let nc = format!("{:08x}", self.nc);
            let cnonce = random_hex(16);
            let resp = format!(
                "{:x}",
                md5::compute(format!("{ha1}:{}:{nc}:{cnonce}:auth:{ha2}", self.nonce))
            );
            v.push_str(&format!(", response=\"{resp}\", qop=auth, nc={nc}, cnonce=\"{cnonce}\""));
        } else {
            let resp = format!("{:x}", md5::compute(format!("{ha1}:{}:{ha2}", self.nonce)));
            v.push_str(&format!(", response=\"{resp}\""));
        }
        v.push_str(", algorithm=MD5");
        if let Some(o) = &self.opaque {
            v.push_str(&format!(", opaque=\"{o}\""));
        }
        let name = if self.proxy { "Proxy-Authorization" } else { "Authorization" };
        (name.to_string(), v)
    }
}

/// Everything needed to (re)build an outgoing request.
#[derive(Clone, Debug)]
pub struct RequestSpec {
    pub method: String,
    pub uri: String,
    pub from: String,
    pub to: String,
    pub call_id: String,
    pub cseq: u32,
    pub contact: Option<String>,
    pub extra: Vec<(String, String)>,
    pub content_type: Option<String>,
    pub body: Vec<u8>,
    pub auth: Option<(String, String)>,
}

pub const USER_AGENT: &str = "TFW-SipModem/1.0";
pub const ALLOW: &str = "INVITE, ACK, CANCEL, BYE, OPTIONS, MESSAGE, INFO, NOTIFY, UPDATE";

impl RequestSpec {
    pub fn render(&self, via_host: &str, via_port: u16, branch: &str) -> Vec<u8> {
        let mut s = format!("{} {} SIP/2.0\r\n", self.method, self.uri);
        s.push_str(&format!("Via: SIP/2.0/UDP {via_host}:{via_port};branch={branch};rport\r\n"));
        s.push_str("Max-Forwards: 70\r\n");
        s.push_str(&format!("From: {}\r\n", self.from));
        s.push_str(&format!("To: {}\r\n", self.to));
        s.push_str(&format!("Call-ID: {}\r\n", self.call_id));
        s.push_str(&format!("CSeq: {} {}\r\n", self.cseq, self.method));
        for (k, v) in &self.extra {
            s.push_str(&format!("{k}: {v}\r\n"));
        }
        if let Some(c) = &self.contact {
            if self.method != "CANCEL" && self.method != "ACK" {
                s.push_str(&format!("Contact: {c}\r\n"));
            }
        }
        if let Some((k, v)) = &self.auth {
            if self.method != "CANCEL" {
                s.push_str(&format!("{k}: {v}\r\n"));
            }
        }
        if self.method != "ACK" && self.method != "CANCEL" {
            s.push_str(&format!("Allow: {ALLOW}\r\n"));
        }
        s.push_str(&format!("User-Agent: {USER_AGENT}\r\n"));
        if let Some(ct) = &self.content_type {
            if !self.body.is_empty() {
                s.push_str(&format!("Content-Type: {ct}\r\n"));
            }
        }
        s.push_str(&format!("Content-Length: {}\r\n\r\n", self.body.len()));
        let mut out = s.into_bytes();
        out.extend_from_slice(&self.body);
        out
    }
}

/// Build a response to a request.
pub fn build_response(
    req: &SipMsg,
    code: u16,
    reason: &str,
    to_tag: Option<&str>,
    extra: &[(&str, String)],
    content_type: Option<&str>,
    body: &[u8],
) -> Vec<u8> {
    let mut s = format!("SIP/2.0 {code} {reason}\r\n");
    for (k, v) in &req.headers {
        if k == "via" {
            s.push_str(&format!("Via: {v}\r\n"));
        }
    }
    for (k, v) in &req.headers {
        if k == "record-route" && (200..300).contains(&code) {
            s.push_str(&format!("Record-Route: {v}\r\n"));
        }
    }
    s.push_str(&format!("From: {}\r\n", req.header("from").unwrap_or("")));
    let to = req.header("to").unwrap_or("").to_string();
    let to = match (tag_of(&to), to_tag) {
        (None, Some(t)) if code > 100 => format!("{to};tag={t}"),
        _ => to,
    };
    s.push_str(&format!("To: {to}\r\n"));
    s.push_str(&format!("Call-ID: {}\r\n", req.call_id()));
    s.push_str(&format!("CSeq: {}\r\n", req.header("cseq").unwrap_or("")));
    for (k, v) in extra {
        s.push_str(&format!("{k}: {v}\r\n"));
    }
    s.push_str(&format!("User-Agent: {USER_AGENT}\r\n"));
    if let Some(ct) = content_type {
        s.push_str(&format!("Content-Type: {ct}\r\n"));
    }
    s.push_str(&format!("Content-Length: {}\r\n\r\n", body.len()));
    let mut out = s.into_bytes();
    out.extend_from_slice(body);
    out
}

/// What we learned from an SDP body.
#[derive(Clone, Debug, Default)]
pub struct SdpInfo {
    pub addr: String,
    pub port: u16,
    pub payloads: Vec<u8>,
    pub clearmode_pt: Option<u8>,
}

pub fn parse_sdp(body: &[u8]) -> Option<SdpInfo> {
    let text = String::from_utf8_lossy(body);
    let mut info = SdpInfo::default();
    let mut session_addr = String::new();
    let mut media_addr = String::new();
    let mut in_audio = false;
    let mut found = false;
    for line in text.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("c=") {
            let a = rest.split_whitespace().nth(2).unwrap_or("").to_string();
            if in_audio {
                media_addr = a;
            } else {
                session_addr = a;
            }
        } else if let Some(rest) = line.strip_prefix("m=") {
            let mut p = rest.split_whitespace();
            let kind = p.next().unwrap_or("");
            in_audio = false;
            if !found && (kind == "audio" || kind == "application" || kind == "image") {
                in_audio = true;
                found = true;
                info.port = p.next().and_then(|s| s.parse().ok()).unwrap_or(0);
                let _proto = p.next();
                info.payloads = p.filter_map(|s| s.parse().ok()).collect();
            }
        } else if let Some(rest) = line.strip_prefix("a=rtpmap:") {
            if in_audio {
                let mut p = rest.split_whitespace();
                let pt: Option<u8> = p.next().and_then(|s| s.parse().ok());
                let enc = p.next().unwrap_or("").to_ascii_uppercase();
                if enc.starts_with("CLEARMODE") {
                    info.clearmode_pt = pt;
                }
            }
        }
    }
    if !found {
        return None;
    }
    info.addr = if media_addr.is_empty() { session_addr } else { media_addr };
    Some(info)
}

pub fn build_sdp(ip: &str, port: u16, pt: u8, clearmode: bool, version: u32) -> Vec<u8> {
    let id = random_hex(8);
    let id: u64 = u64::from_str_radix(&id, 16).unwrap_or(1) % 1_000_000_000;
    let rtpmap = if clearmode {
        format!("a=rtpmap:{pt} CLEARMODE/8000\r\n")
    } else {
        format!("a=rtpmap:{pt} PCMU/8000\r\n")
    };
    format!(
        "v=0\r\no=tfw {id} {version} IN IP4 {ip}\r\ns=TFW SIP Modem\r\nc=IN IP4 {ip}\r\nt=0 0\r\nm=audio {port} RTP/AVP {pt}\r\n{rtpmap}a=ptime:20\r\na=sendrecv\r\n"
    )
    .into_bytes()
}

