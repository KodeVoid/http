use std::collections::HashMap;
use std::io::{Result, Write};

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: None,
            body: None,
        }
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> Self {
        let mut response: HttpResponse<'a> = HttpResponse::default();
        
        response.status_code = status_code;
        
        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };
        
        response.status_text = match response.status_code {
            "200" => "OK",
            "400" => "Bad Request",
            "404" => "Not Found",
            "500" => "Internal Server Error",
            _ => "Not Found",
        };
        
        response.body = body;
        response
    }

    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
        let response_string = String::from(self.clone());
        write!(write_stream, "{}", response_string)?;
        Ok(())
    }

    fn version(&self) -> &str {
        self.version
    }
    
    fn status_code(&self) -> &str {
        self.status_code
    }
    
    fn status_text(&self) -> &str {
        self.status_text
    }
    
    fn headers(&self) -> String {
        match &self.headers {
            Some(map) => {
                let mut header_string = String::new();
                for (k, v) in map.iter() {
                    header_string = format!("{}{}: {}\r\n", header_string, k, v);
                }
                header_string
            }
            None => String::new(),
        }
    }
    
    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }
}

impl<'a> From<HttpResponse<'a>> for String {
    fn from(res: HttpResponse<'a>) -> Self {
        let body_length = match &res.body {
            Some(b) => b.len(),
            None => 0,
        };
        
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            res.version(),
            res.status_code(),
            res.status_text(),
            res.headers(),
            body_length,
            res.body()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_response_struct_creation_200() {
        let response_actual = HttpResponse::new(
            "200",
            None,
            Some("Items was testing fine as of 1st August 2025".to_string()),
        );
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("Items was testing fine as of 1st August 2025".to_string()),
        };
        assert_eq!(response_actual, response_expected);
    }
    
    #[test]
    fn test_response_struct_creation_404() {
        let response_actual = HttpResponse::new(
            "404",
            None,
            Some("Item was shipped on 21st Dec 2020".to_string()),
        );
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("Item was shipped on 21st Dec 2020".to_string()),
        };
        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn test_http_response_creation() {
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("Item was shipped on 21st Dec 2020".to_string()),
        };
        let http_string: String = response_expected.into();
        let response_actual = "HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\nContent-Length: 33\r\n\r\nItem was shipped on 21st Dec 2020";
        assert_eq!(http_string, response_actual);
    }
    
    #[test]
    fn test_response_with_no_body() {
        let response = HttpResponse::new("200", None, None);
        let http_string: String = response.into();
        let expected = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: 0\r\n\r\n";
        assert_eq!(http_string, expected);
    }
    
    #[test]
    fn test_custom_headers() {
        let mut custom_headers = HashMap::new();
        custom_headers.insert("Content-Type", "application/json");
        custom_headers.insert("Cache-Control", "no-cache");
        
        let body_content = "{\"message\": \"success\"}";
        let response = HttpResponse::new(
            "200",
            Some(custom_headers),
            Some(body_content.to_string()),
        );
        
        let http_string: String = response.into();
        assert!(http_string.contains("Content-Type: application/json"));
        assert!(http_string.contains("Cache-Control: no-cache"));
        assert!(http_string.contains(&format!("Content-Length: {}", body_content.len())));
    }
}