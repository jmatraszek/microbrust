pub fn to_hex_string(bytes: &[u8]) -> String {
  let strs: Vec<String> = bytes.iter()
                               .map(|b| format!("{:02x}", b))
                               .collect();
  strs.join(" ")
}
