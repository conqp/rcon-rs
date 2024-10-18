pub trait IntoBytes: Sized {
    fn into_bytes(self) -> impl AsRef<[u8]>;
}
