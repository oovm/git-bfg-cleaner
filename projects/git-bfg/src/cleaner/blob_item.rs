use super::*;


impl Display for BlobItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let size = Byte::from_bytes(self.size as u128).get_appropriate_unit(false).to_string();
        write!(f, "{:>9} | {} | {:?}", size, self.id, self.format)
    }
}


impl Display for BlobFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Binary => { f.write_char('b') }
            Self::Text => { f.write_char('t') }
        }
    }
}

impl BlobFormat {
    pub fn from_blob(blob: &Blob) -> Self {
        match blob.is_binary() {
            true => { Self::Binary }
            false => { Self::Text }
        }
    }
}

impl Eq for BlobItem {}

impl PartialEq<Self> for BlobItem {
    fn eq(&self, other: &Self) -> bool {
        self.size.eq(&other.size)
    }
}

impl PartialOrd<Self> for BlobItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.size.partial_cmp(&other.size)
    }
}

impl Ord for BlobItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.size.cmp(&other.size)
    }
}