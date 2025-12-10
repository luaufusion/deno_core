pub(crate) mod sealed {
    pub trait CreateBuffer {}
    impl CreateBuffer for u8 {}
    impl CreateBuffer for i8 {}
    impl CreateBuffer for u16 {}
    impl CreateBuffer for i16 {}
    impl CreateBuffer for u32 {}
    impl CreateBuffer for i32 {}
    impl CreateBuffer for f32 {}
    impl CreateBuffer for f64 {}
    impl CreateBuffer for u64 {}
    impl CreateBuffer for i64 {}
}

/// Helper method to create a V8 BackingStore from a boxed u8 slice,
pub fn v8_create_backing_store<'s, 'i, T: sealed::CreateBuffer>(
  scope: &mut v8::PinScope<'s, 'i>,
  buf: &[T],
  buf_len: usize,
) -> v8::UniqueRef<v8::BackingStore> {
    #[cfg(not(feature = "v8_enable_sandbox"))]
    {
        return v8::ArrayBuffer::new_backing_store_from_bytes(scope, buf);
    }
    #[cfg(feature = "v8_enable_sandbox")]
    {
        let backing_store = v8::ArrayBuffer::new_backing_store(scope, buf_len * std::mem::size_of::<T>());
        // Copy the data into the backing store
        unsafe {
            if let Some(data) = backing_store.data() {
            std::ptr::copy(
                buf.as_ptr(),
                data.as_ptr() as *mut T,
                buf_len,
            );
            };
        }
        backing_store
    }
}
