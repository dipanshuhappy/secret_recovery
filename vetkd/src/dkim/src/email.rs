use std::error::Error;
use viadkim::header::HeaderFields;

#[derive(Debug)]
pub struct EmailMessage {
    pub header_string: String,
    pub body_string: String,
    pub header_fields: HeaderFields,
}

// For now, support only UTF-8 input, and normalise line endings to CRLF.
pub fn parse_message(s: &str) -> Result<EmailMessage, Box<dyn Error>> {
    let lines: Vec<_> = s.lines().collect();
    let mut msg = lines.join("\r\n");
    msg.push_str("\r\n");

    let i = msg.find("\r\n\r\n").ok_or("ill-formed input message")?;

    let mut header: String = msg.drain(..i + 4).collect();
    header.pop();
    header.pop();

    let body = msg;

    let headers = header.parse().map_err(|_| "ill-formed input message")?;

    Ok(EmailMessage {
        header_string: header,
        body_string: body,
        header_fields: headers,
    })
}
