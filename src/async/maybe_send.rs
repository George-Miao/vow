use cfg_if::cfg_if;

cfg_if! {

    if #[cfg(any(feature = "backend-tokio", feature = "send"))] {
        #[doc(hidden)]
        pub trait MaybeSend: Send {}
        impl<T: Send + ?Sized> MaybeSend for T {}
    } else {
        #[doc(hidden)]
        pub trait MaybeSend {}
        impl<T: ?Sized> MaybeSend for T {}
    }
}
